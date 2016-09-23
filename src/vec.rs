// Copyright 2014 The Prometheus Authors
// Copyright 2016 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::hash::Hasher;

use fnv::FnvHasher;
use protobuf::RepeatedField;

use desc::{Describer, Desc};
use metrics::{Collector, Metric};
use proto::{MetricFamily, MetricType};
use errors::{Result, Error};

/// `MetricVecBuilder` is the trait to build a metric.
pub trait MetricVecBuilder: Send + Sync + Clone {
    type M: Metric;
    type P: Describer + Sync + Send + Clone;

    /// `build` builds a Metric with option and corresponding label names.
    fn build(&self, &Self::P, &[&str]) -> Result<Self::M>;
}

pub struct Entry<K, V> {
    key: K,
    value: V,
}

impl<K, V> Entry<K, V> {
    #[inline]
    pub fn new(key: K, value: V) -> Entry<K, V> {
        Entry {
            key: key,
            value: value,
        }
    }

    #[inline]
    pub fn key(&self) -> &K {
        &self.key
    }

    #[inline]
    pub fn value(&self) -> &V {
        &self.value
    }

    #[inline]
    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    #[inline]
    pub fn take_value(self) -> V {
        self.value
    }
}

pub struct MapVec<K: Eq, V> {
    entries: Vec<Entry<K, V>>,
}

impl<K: Eq, V> MapVec<K, V> {
    pub fn new() -> MapVec<K, V> {
        MapVec { entries: Vec::new() }
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.entries
            .iter()
            .find(|&v| k == v.key())
            .map(|entry| entry.value())
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let idx = self.entries
            .iter()
            .position(|v| &k == v.key());

        let entry = Entry::new(k, v);
        self.entries.push(entry);
        idx.map(|idx| self.entries.swap_remove(idx).take_value())
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        self.entries
            .iter()
            .position(|v| k == v.key())
            .map(|idx| self.entries.remove(idx).take_value())
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn get_vec(&self) -> &Vec<Entry<K, V>> {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

struct MetricVecCore<T: MetricVecBuilder> {
    pub children: RwLock<MapVec<u64, T::M>>,
    pub desc: Desc,
    pub metric_type: MetricType,
    pub new_metric: T,
    pub opts: T::P,
}

impl<T: MetricVecBuilder> MetricVecCore<T> {
    pub fn desc(&self) -> &Desc {
        &self.desc
    }

    pub fn collect(&self) -> MetricFamily {
        let mut m = MetricFamily::new();
        m.set_name(self.desc.fq_name.clone());
        m.set_help(self.desc.help.clone());
        m.set_field_type(self.metric_type);

        let children = self.children.read().unwrap();
        let mut metrics = Vec::with_capacity(children.len());
        for child in children.get_vec() {
            metrics.push(child.value().metric());
        }
        m.set_metric(RepeatedField::from_vec(metrics));
        m
    }

    pub fn get_metric_with_label_values(&self, vals: &[&str]) -> Result<T::M> {
        let h = try!(self.hash_label_values(&vals));

        if let Some(metric) = self.children.read().unwrap().get(&h) {
            return Ok(metric.clone());
        }

        self.get_or_create_metric(h, vals)
    }

    pub fn get_metric_with(&self, labels: &HashMap<&str, &str>) -> Result<T::M> {
        let h = try!(self.hash_labels(labels));

        if let Some(metric) = self.children.read().unwrap().get(&h) {
            return Ok(metric.clone());
        }

        let mut vals: Vec<&str> = labels.values().map(|v| v.as_ref()).collect();
        vals.sort_by(|v1, v2| v1.cmp(v2));
        self.get_or_create_metric(h, &vals)
    }

    pub fn delete_label_values(&self, vals: &[&str]) -> Result<()> {
        let h = try!(self.hash_label_values(&vals));

        let mut children = self.children.write().unwrap();
        if children.remove(&h).is_none() {
            return Err(Error::Msg(format!("missing label values {:?}", vals)));
        }

        Ok(())
    }

    pub fn delete(&self, labels: &HashMap<&str, &str>) -> Result<()> {
        let h = try!(self.hash_labels(labels));

        let mut children = self.children.write().unwrap();
        if children.remove(&h).is_none() {
            return Err(Error::Msg(format!("missing labels {:?}", labels)));
        }

        Ok(())
    }

    /// `reset` deletes all metrics in this vector.
    pub fn reset(&self) {
        self.children.write().unwrap().clear();
    }

    fn hash_label_values(&self, vals: &[&str]) -> Result<u64> {
        if vals.len() != self.desc.variable_labels.len() {
            return Err(Error::InconsistentCardinality(self.desc.variable_labels.len(), vals.len()));
        }

        let mut h = FnvHasher::default();
        for val in vals {
            h.write(val.as_bytes());
        }

        Ok(h.finish())
    }

    fn hash_labels(&self, labels: &HashMap<&str, &str>) -> Result<u64> {
        if labels.len() != self.desc.variable_labels.len() {
            return Err(Error::InconsistentCardinality(self.desc.variable_labels.len(),
                                                      labels.len()));
        }

        let mut h = FnvHasher::default();
        for label in &self.desc.variable_labels {
            match labels.get(&label.as_ref()) {
                Some(val) => h.write(val.as_bytes()),
                None => {
                    return Err(Error::Msg(format!("label name {} missing in label map", label)))
                }
            }

        }

        Ok(h.finish())
    }

    fn get_or_create_metric(&self, hash: u64, label_values: &[&str]) -> Result<T::M> {
        let mut children = self.children.write().unwrap();
        // Check exist first.
        if let Some(metric) = children.get(&hash) {
            return Ok(metric.clone());
        }

        let metric = try!(self.new_metric.build(&self.opts, label_values));
        children.insert(hash, metric.clone());
        Ok(metric)
    }
}

/// `MetricVec` is a Collector to bundle metrics of the same name that
/// differ in their label values. `MetricVec` is usually not used directly but as a
/// building block for implementations of vectors of a given metric
/// type. `GaugeVec`, `CounterVec`, `SummaryVec`, and `UntypedVec` are examples already
/// provided in this package.
#[derive(Clone)]
pub struct MetricVec<T: MetricVecBuilder> {
    v: Arc<MetricVecCore<T>>,
}

impl<T: MetricVecBuilder> MetricVec<T> {
    /// `create` creates a MetricVec with description `desc`, a metric type `metric_type` and
    /// a MetricVecBuilder `new_metric`.
    pub fn create(metric_type: MetricType, new_metric: T, opts: T::P) -> Result<MetricVec<T>> {
        let desc = try!(opts.describe());
        let v = MetricVecCore {
            children: RwLock::new(MapVec::new()),
            desc: desc,
            metric_type: metric_type,
            new_metric: new_metric,
            opts: opts,
        };

        Ok(MetricVec { v: Arc::new(v) })
    }

    /// `get_metric_with_label_values` returns the Metric for the given slice of label
    /// values (same order as the VariableLabels in Desc). If that combination of
    /// label values is accessed for the first time, a new Metric is created.
    ///
    /// It is possible to call this method without using the returned Metric to only
    /// create the new Metric but leave it at its start value (e.g. a Summary or
    /// Histogram without any observations). See also the SummaryVec example.
    ///
    /// Keeping the Metric for later use is possible (and should be considered if
    /// performance is critical), but keep in mind that Reset, DeleteLabelValues and
    /// Delete can be used to delete the Metric from the MetricVec. In that case, the
    /// Metric will still exist, but it will not be exported anymore, even if a
    /// Metric with the same label values is created later. See also the CounterVec
    /// example.
    ///
    /// An error is returned if the number of label values is not the same as the
    /// number of VariableLabels in Desc.
    ///
    /// Note that for more than one label value, this method is prone to mistakes
    /// caused by an incorrect order of arguments. Consider get_metric_with(labels) as
    /// an alternative to avoid that type of mistake. For higher label numbers, the
    /// latter has a much more readable (albeit more verbose) syntax, but it comes
    /// with a performance overhead (for creating and processing the Labels map).
    pub fn get_metric_with_label_values(&self, vals: &[&str]) -> Result<T::M> {
        self.v.get_metric_with_label_values(vals)
    }

    /// `get_metric_with` returns the Metric for the given Labels map (the label names
    /// must match those of the VariableLabels in Desc). If that label map is
    /// accessed for the first time, a new Metric is created. Implications of
    /// creating a Metric without using it and keeping the Metric for later use are
    /// the same as for GetMetricWithLabelValues.
    ///
    /// An error is returned if the number and names of the Labels are inconsistent
    /// with those of the VariableLabels in Desc.
    ///
    /// This method is used for the same purpose as
    /// `get_metric_with_label_values`. See there for pros and cons of the two
    /// methods.
    pub fn get_metric_with(&self, labels: &HashMap<&str, &str>) -> Result<T::M> {
        self.v.get_metric_with(labels)
    }

    /// `with_label_values` works as `get_metric_with_label_values`, but panics if an error
    /// occurs. The method allows neat syntax like:
    ///     httpReqs.with_label_values("404", "POST").inc()
    pub fn with_label_values(&self, vals: &[&str]) -> T::M {
        self.get_metric_with_label_values(vals).unwrap()
    }

    /// `with` works as `get_metric_with`, but panics if an error occurs. The method allows
    /// neat syntax like:
    ///     httpReqs.with(Labels{"status":"404", "method":"POST"}).inc()
    pub fn with(&self, labels: &HashMap<&str, &str>) -> T::M {
        self.get_metric_with(labels).unwrap()
    }

    /// `remove_label_values` removes the metric where the variable labels are the same
    /// as those passed in as labels (same order as the VariableLabels in Desc). It
    /// returns true if a metric was deleted.
    ///
    /// It returns an error if the number of label values is not the same as the
    /// number of VariableLabels in Desc.
    ///
    /// Note that for more than one label value, this method is prone to mistakes
    /// caused by an incorrect order of arguments. Consider delete(labels) as an
    /// alternative to avoid that type of mistake. For higher label numbers, the
    /// latter has a much more readable (albeit more verbose) syntax, but it comes
    /// with a performance overhead (for creating and processing the Labels map).
    pub fn remove_label_values(&self, vals: &[&str]) -> Result<()> {
        self.v.delete_label_values(vals)
    }

    /// `remove` removes the metric where the variable labels are the same as those
    /// passed in as labels. It returns true if a metric was deleted.
    ///
    /// It returns an error if the number and names of the Labels are inconsistent
    /// with those of the VariableLabels in the Desc of the MetricVec.
    ///
    /// This method is used for the same purpose as `delete_label_values`. See
    /// there for pros and cons of the two methods.
    pub fn remove(&self, labels: &HashMap<&str, &str>) -> Result<()> {
        self.v.delete(labels)
    }


    /// `reset` deletes all metrics in this vector.
    pub fn reset(&self) {
        self.v.reset()
    }
}


impl<T: MetricVecBuilder> Collector for MetricVec<T> {
    fn desc(&self) -> &Desc {
        &self.v.desc
    }

    fn collect(&self) -> MetricFamily {
        self.v.collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapvec() {
        let mut mv = MapVec::new();

        assert!(mv.get(&0xdeadbeef_u64).is_none());

        mv.insert(0xdeadbeef_u64, "0xdeadbeef_u64".to_owned());
        assert!(mv.get(&0xdeadbeef_u64).is_some());
        assert_eq!(mv.get(&0xdeadbeef_u64).unwrap(),
                   &"0xdeadbeef_u64".to_owned());

        mv.insert(0xdeadbeef_u64, "0xbeefdead".to_owned());
        assert!(mv.get(&0xdeadbeef_u64).is_some());
        assert_eq!(mv.get(&0xdeadbeef_u64).unwrap(), &"0xbeefdead".to_owned());
        assert_eq!(mv.get_vec().len(), 1);

        mv.insert(0xfabaceae_u64, "0xfabaceae_u64".to_owned());
        assert!(mv.get(&0xfabaceae_u64).is_some());
        assert_eq!(mv.get(&0xfabaceae_u64).unwrap(),
                   &"0xfabaceae_u64".to_owned());

        assert!(mv.get(&0xdeadbeef_u64).is_some());
        assert_eq!(mv.get(&0xdeadbeef_u64).unwrap(), &"0xbeefdead".to_owned());
        assert_eq!(mv.get_vec().len(), 2);

        let deadbeef = mv.remove(&0xdeadbeef_u64);
        assert!(deadbeef.is_some());
        assert_eq!(deadbeef.unwrap(), "0xbeefdead".to_owned());
        assert_eq!(mv.get_vec().len(), 1);

        mv.clear();
        assert!(mv.remove(&0xdeadbeef_u64).is_none());
        assert!(mv.remove(&0xfabaceae_u64).is_none());
        assert_eq!(mv.get_vec().len(), 0);
    }
}
