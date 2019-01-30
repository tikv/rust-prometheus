use std::sync::Arc;

use crossbeam_utils::sync::ShardedLock;
use hashbrown::hash_map::{HashMap, Entry};

use crate::atomic::{Atomic, AtomicInt};
use crate::{Error, Result};

#[derive(Clone)]
struct CounterCore(Arc<AtomicInt>);

impl CounterCore {
    pub fn new() -> Self {
        Self(Arc::new(AtomicInt::zero()))
    }

    pub fn inc(&self) {
        self.0.inc();
    }

    pub fn inc_by(&self, val: isize) {
        debug_assert!(val >= 0);
        self.0.inc_by(val);
    }

    pub fn get(&self) -> isize {
        self.0.get()
    }
}

struct CounterInner {
    desc: Descriptor,
    core: CounterCore,
}

#[derive(Clone)]
pub struct Counter(Arc<CounterInner>);

impl Counter {
    pub fn inc(&self) {
        self.0.core.inc();
    }

    pub fn inc_by(&self, val: isize) {
        self.0.core.inc_by(val);
    }

    pub fn get(&self) -> isize {
        self.0.core.get()
    }
}

impl crate::Collector for Counter {
    fn box_clone(&self) -> Box<crate::Collector> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct ChildCounter(CounterCore);

impl ChildCounter {
    pub fn inc(&self) {
        self.0.inc();
    }

    pub fn inc_by(&self, val: isize) {
        self.0.inc_by(val);
    }

    pub fn get(&self) -> isize {
        self.0.get()
    }
}

pub struct CounterVecInner<Len: crate::LabelLengthPlaceholder> {
    desc: Descriptor,
    children: ShardedLock<HashMap<u64, ChildCounter>>,
    _phantom: std::marker::PhantomData<Len>,
}

pub struct CounterVec<Len: crate::LabelLengthPlaceholder>(Arc<CounterVecInner<Len>>);

impl<Len: crate::LabelLengthPlaceholder> Clone for CounterVec<Len> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Len: crate::LabelLengthPlaceholder> crate::Collector for CounterVec<Len> {
    fn box_clone(&self) -> Box<crate::Collector> {
        Box::new(self.clone())
    }
}

impl<Len: crate::LabelLengthPlaceholder> CounterVec<Len> {
    fn hash_label_values<'a>(values: impl std::iter::Iterator<Item=&'a [u8]>) -> u64 {
        use std::hash::Hasher;

        let mut h = fxhash::FxHasher::default();
        for value in values {
            h.write(value);
        }
        h.finish()
    }

    pub fn with_label_values(&self, values: impl crate::AsLabelValuesIter<Len>) -> ChildCounter {
        let hash = Self::hash_label_values(values.values_iter().map(|t| t.as_ref().as_bytes()));

        // First, try read using a read lock.
        if let Some(m) = self.0.children.read().unwrap().get(&hash) {
            return m.clone();
        }

        // Next, grab a write lock and let's try read again.
        let mut children = self.0.children.write().unwrap();
        match children.entry(hash) {
            Entry::Occupied(o) => o.get().clone(),
            Entry::Vacant(v) => {
                let metric = ChildCounter(CounterCore::new()); // TODO: Build child counter
                v.insert(metric).clone()
            }
        }
    }

    pub fn delete_label_values(&self, values: impl crate::AsLabelValuesIter<Len>) -> Result<()> {
        let hash = Self::hash_label_values(values.values_iter().map(|t| t.as_ref().as_bytes()));
        let mut children = self.0.children.write().unwrap();
        if children.remove(&hash).is_none() {
            // TODO: Return Error instead of panic
            panic!();
        }

        Ok(())
    }
}

pub struct CounterBuilder<'a> {
    name: String,
    help: String,
    const_labels: Vec<(String, String)>,
    registry: Option<&'a mut crate::Registry>,
}

impl CounterBuilder<'static> {
    pub fn new(name: impl Into<String>, help: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            help: help.into(),
            const_labels: Vec::new(),
            registry: None
        }
    }
}

impl<'a> CounterBuilder<'a> {
    pub fn registry(mut self, registry: &mut crate::Registry) -> CounterBuilder {
        CounterBuilder {
            name: self.name,
            help: self.help,
            const_labels: self.const_labels,
            registry: Some(registry),
        }
    }

    pub fn const_label(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.const_labels.push((name.into(), value.into()));
        self
    }

    pub fn build(self) -> Result<Counter> {
        let desc = Descriptor::new(self.name, self.help, self.const_labels, vec![])?;
        let inner = CounterInner {
            desc,
            core: CounterCore::new(),
        };
        let collector = Counter(Arc::new(inner));
        if let Some(reg) = self.registry {
            reg.register(&collector)?;
        }
        Ok(collector)
    }

    pub fn build_vec<T>(self, label_names: T) -> Result<CounterVec<T::Len>>
        where
            T: crate::IntoLabelVec
    {
        let desc = Descriptor::new(self.name, self.help, self.const_labels, label_names.into())?;
        let inner = CounterVecInner {
            desc,
            children: ShardedLock::new(HashMap::default()),
            _phantom: std::marker::PhantomData,
        };
        let collector = CounterVec(Arc::new(inner));
        if let Some(reg) = self.registry {
            reg.register(&collector)?;
        }
        Ok(collector)
    }
}
