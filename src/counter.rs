use std::sync::Arc;

use crossbeam_utils::sync::ShardedLock;
use hashbrown::hash_map::{Entry, HashMap};

use crate::atomic::{Atomic, AtomicFloat, AtomicInt};
use crate::{Error, Result};

struct GeneralCounterCore<A: Atomic>(Arc<A>);

impl<A: Atomic> Clone for GeneralCounterCore<A> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A: Atomic> GeneralCounterCore<A> {
    #[inline]
    pub fn new() -> Self {
        Self(Arc::new(A::zero()))
    }

    #[inline]
    pub fn inc(&self) {
        self.0.inc();
    }

    #[inline]
    pub fn inc_by(&self, val: A::Num) {
        use crate::atomic::Number;
        debug_assert!(val.is_not_negative());
        self.0.inc_by(val);
    }

    #[inline]
    pub fn get(&self) -> A::Num {
        self.0.get()
    }
}

struct GeneralCounterInner<A: Atomic> {
    desc: crate::descriptor::Descriptor,
    core: GeneralCounterCore<A>,
}

pub struct GeneralCounter<A: Atomic>(Arc<GeneralCounterInner<A>>);

impl<A: Atomic> Clone for GeneralCounter<A> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A: Atomic> GeneralCounter<A> {
    #[inline]
    pub fn inc(&self) {
        self.0.core.inc();
    }

    #[inline]
    pub fn inc_by(&self, val: A::Num) {
        self.0.core.inc_by(val);
    }

    #[inline]
    pub fn get(&self) -> A::Num {
        self.0.core.get()
    }
}

impl<A: Atomic> crate::Collector for GeneralCounter<A> {
    #[inline]
    fn describe(&self) -> Vec<&crate::Descriptor> {
        vec![&self.0.desc]
    }

    #[inline]
    fn box_clone(&self) -> Box<crate::Collector> {
        Box::new(self.clone())
    }
}

pub struct GeneralChildCounter<A: Atomic>(GeneralCounterCore<A>);

impl<A: Atomic> Clone for GeneralChildCounter<A> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A: Atomic> GeneralChildCounter<A> {
    #[inline]
    pub fn inc(&self) {
        self.0.inc();
    }

    #[inline]
    pub fn inc_by(&self, val: A::Num) {
        self.0.inc_by(val);
    }

    #[inline]
    pub fn get(&self) -> A::Num {
        self.0.get()
    }
}

pub struct GeneralGeneralCounterVecInner<A: Atomic, Len: crate::LabelLengthPlaceholder> {
    desc: crate::descriptor::Descriptor,
    children: ShardedLock<HashMap<u64, GeneralChildCounter<A>>>,
    _phantom: std::marker::PhantomData<Len>,
}

pub struct GeneralCounterVec<A: Atomic, Len: crate::LabelLengthPlaceholder>(
    Arc<GeneralGeneralCounterVecInner<A, Len>>,
);

impl<A: Atomic, Len: crate::LabelLengthPlaceholder> Clone for GeneralCounterVec<A, Len> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A: Atomic, Len: crate::LabelLengthPlaceholder> crate::Collector for GeneralCounterVec<A, Len> {
    #[inline]
    fn describe(&self) -> Vec<&crate::Descriptor> {
        vec![&self.0.desc]
    }

    #[inline]
    fn box_clone(&self) -> Box<crate::Collector> {
        Box::new(self.clone())
    }
}

impl<A: Atomic, Len: crate::LabelLengthPlaceholder> GeneralCounterVec<A, Len> {
    #[inline]
    fn hash_label_values<'a>(values: impl std::iter::Iterator<Item = &'a [u8]>) -> u64 {
        use std::hash::Hasher;

        let mut h = fxhash::FxHasher::default();
        for value in values {
            h.write(value);
        }
        h.finish()
    }

    pub fn with_label_values(
        &self,
        values: impl crate::AsLabelValuesIter<Len>,
    ) -> GeneralChildCounter<A> {
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
                let metric = GeneralChildCounter(GeneralCounterCore::new()); // TODO: Build child counter
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

pub type Counter = GeneralCounter<AtomicInt>;
pub type CounterVec<Len> = GeneralCounterVec<AtomicInt, Len>;

pub type IntCounter = GeneralCounter<AtomicFloat>;
pub type IntCounterVec<Len> = GeneralCounterVec<AtomicFloat, Len>;

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
            registry: None,
        }
    }
}

impl<'a> CounterBuilder<'a> {
    pub fn registry(self, registry: &mut crate::Registry) -> CounterBuilder {
        CounterBuilder {
            name: self.name,
            help: self.help,
            const_labels: self.const_labels,
            registry: Some(registry),
        }
    }

    #[inline]
    pub fn const_label(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.const_labels.push((name.into(), value.into()));
        self
    }

    #[inline]
    fn build_impl<A>(self) -> Result<GeneralCounter<A>>
    where
        A: Atomic,
    {
        let desc =
            crate::descriptor::Descriptor::new(self.name, self.help, self.const_labels, vec![])?;
        let inner = GeneralCounterInner {
            desc,
            core: GeneralCounterCore::new(),
        };
        let collector = GeneralCounter(Arc::new(inner));
        if let Some(reg) = self.registry {
            reg.register(&collector)?;
        }
        Ok(collector)
    }

    #[inline]
    fn build_vec_impl<A, T>(self, label_names: T) -> Result<GeneralCounterVec<A, T::Len>>
    where
        A: Atomic,
        T: crate::IntoLabelVec,
    {
        let desc = crate::descriptor::Descriptor::new(
            self.name,
            self.help,
            self.const_labels,
            label_names.into(),
        )?;
        let inner = GeneralGeneralCounterVecInner {
            desc,
            children: ShardedLock::new(HashMap::default()),
            _phantom: std::marker::PhantomData,
        };
        let collector = GeneralCounterVec(Arc::new(inner));
        if let Some(reg) = self.registry {
            reg.register(&collector)?;
        }
        Ok(collector)
    }

    pub fn build(self) -> Result<Counter> {
        self.build_impl()
    }

    pub fn build_int(self) -> Result<IntCounter> {
        self.build_impl()
    }

    pub fn build_vec<T>(self, label_names: T) -> Result<CounterVec<T::Len>>
    where
        T: crate::IntoLabelVec,
    {
        self.build_vec_impl(label_names)
    }

    pub fn build_int_vec<T>(self, label_names: T) -> Result<IntCounterVec<T::Len>>
    where
        T: crate::IntoLabelVec,
    {
        self.build_vec_impl(label_names)
    }
}
