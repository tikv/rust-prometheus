// Copyright 2014 The Prometheus Authors
// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::collections::HashMap;
use std::hash::Hasher;
use std::sync::Arc;

use fnv::FnvHasher;
use parking_lot::RwLock;

use crate::desc::{Desc, Describer};
use crate::errors::{Error, Result};
use crate::metrics::{Collector, Metric};
use crate::proto::{MetricFamily, MetricType};

/// An interface for building a metric vector.
pub trait MetricVecBuilder: Send + Sync + Clone {
    /// The associated Metric collected.
    type M: Metric;
    /// The associated describer.
    type P: Describer + Sync + Send + Clone;

    /// `build` builds a [`Metric`] with option and corresponding label names.
    fn build(&self, _: &Self::P, _: &[&str]) -> Result<Self::M>;
}

#[derive(Debug)]
pub(crate) struct MetricVecCore<T: MetricVecBuilder> {
    pub children: RwLock<HashMap<u64, T::M>>,
    pub desc: Desc,
    pub metric_type: MetricType,
    pub new_metric: T,
    pub opts: T::P,
}

impl<T: MetricVecBuilder> MetricVecCore<T> {
    pub fn collect(&self) -> MetricFamily {
        let children = self.children.read();
        let mut metrics = Vec::with_capacity(children.len());
        for child in children.values() {
            metrics.push(child.metric());
        }
        MetricFamily {
            name: Some(self.desc.fq_name.clone()),
            help: Some(self.desc.help.clone()),
            r#type: Some(self.metric_type.into()),
            metric: metrics,
        }
    }

    pub fn get_metric_with_label_values(&self, vals: &[&str]) -> Result<T::M> {
        let h = self.hash_label_values(vals)?;

        if let Some(metric) = self.children.read().get(&h).cloned() {
            return Ok(metric);
        }

        self.get_or_create_metric(h, vals)
    }

    pub fn get_metric_with(&self, labels: &HashMap<&str, &str>) -> Result<T::M> {
        let h = self.hash_labels(labels)?;

        if let Some(metric) = self.children.read().get(&h).cloned() {
            return Ok(metric);
        }

        let vals = self.get_label_values(labels)?;
        self.get_or_create_metric(h, &vals)
    }

    pub fn delete_label_values(&self, vals: &[&str]) -> Result<()> {
        let h = self.hash_label_values(vals)?;

        let mut children = self.children.write();
        if children.remove(&h).is_none() {
            return Err(Error::Msg(format!("missing label values {:?}", vals)));
        }

        Ok(())
    }

    pub fn delete(&self, labels: &HashMap<&str, &str>) -> Result<()> {
        let h = self.hash_labels(labels)?;

        let mut children = self.children.write();
        if children.remove(&h).is_none() {
            return Err(Error::Msg(format!("missing labels {:?}", labels)));
        }

        Ok(())
    }

    /// `reset` deletes all metrics in this vector.
    pub fn reset(&self) {
        self.children.write().clear();
    }

    pub(crate) fn hash_label_values(&self, vals: &[&str]) -> Result<u64> {
        if vals.len() != self.desc.variable_labels.len() {
            return Err(Error::InconsistentCardinality {
                expect: self.desc.variable_labels.len(),
                got: vals.len(),
            });
        }

        let mut h = FnvHasher::default();
        for val in vals {
            h.write(val.as_bytes());
        }

        Ok(h.finish())
    }

    fn hash_labels(&self, labels: &HashMap<&str, &str>) -> Result<u64> {
        if labels.len() != self.desc.variable_labels.len() {
            return Err(Error::InconsistentCardinality {
                expect: self.desc.variable_labels.len(),
                got: labels.len(),
            });
        }

        let mut h = FnvHasher::default();
        for name in &self.desc.variable_labels {
            match labels.get(&name.as_ref()) {
                Some(val) => h.write(val.as_bytes()),
                None => {
                    return Err(Error::Msg(format!(
                        "label name {} missing in label map",
                        name
                    )));
                }
            }
        }

        Ok(h.finish())
    }

    fn get_label_values<'a>(&self, labels: &'a HashMap<&str, &str>) -> Result<Vec<&'a str>> {
        let mut values = Vec::new();
        for name in &self.desc.variable_labels {
            match labels.get(&name.as_ref()) {
                Some(val) => values.push(*val),
                None => {
                    return Err(Error::Msg(format!(
                        "label name {} missing in label map",
                        name
                    )));
                }
            }
        }
        Ok(values)
    }

    fn get_or_create_metric(&self, hash: u64, label_values: &[&str]) -> Result<T::M> {
        let mut children = self.children.write();
        // Check exist first.
        if let Some(metric) = children.get(&hash).cloned() {
            return Ok(metric);
        }

        let metric = self.new_metric.build(&self.opts, label_values)?;
        children.insert(hash, metric.clone());
        Ok(metric)
    }
}

/// A [`Collector`] to bundle metrics of the same name that
/// differ in their label values. It is usually not used directly but as a
/// building block for implementations of vectors of a given metric
/// type. [`GaugeVec`](crate::GaugeVec) and [`CounterVec`](crate::CounterVec)
/// are examples already provided in this package.
#[derive(Clone)]
pub struct MetricVec<T: MetricVecBuilder> {
    pub(crate) v: Arc<MetricVecCore<T>>,
}

impl<T: MetricVecBuilder> std::fmt::Debug for MetricVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MetricVec")
    }
}

impl<T: MetricVecBuilder> MetricVec<T> {
    /// `create` creates a MetricVec with description `desc`, a metric type `metric_type` and
    /// a MetricVecBuilder `new_metric`.
    pub fn create(metric_type: MetricType, new_metric: T, opts: T::P) -> Result<MetricVec<T>> {
        let desc = opts.describe()?;
        let v = MetricVecCore {
            children: RwLock::new(HashMap::new()),
            desc,
            metric_type,
            new_metric,
            opts,
        };

        Ok(MetricVec { v: Arc::new(v) })
    }

    /// `get_metric_with_label_values` returns the [`Metric`] for the given slice
    /// of label values (same order as the VariableLabels in Desc). If that combination of
    /// label values is accessed for the first time, a new [`Metric`] is created.
    ///
    /// It is possible to call this method without using the returned [`Metric`]
    /// to only create the new [`Metric`] but leave it at its start value (e.g. a
    /// [`Histogram`](crate::Histogram) without any observations).
    ///
    /// Keeping the [`Metric`] for later use is possible (and should be considered
    /// if performance is critical), but keep in mind that Reset, DeleteLabelValues and Delete can
    /// be used to delete the [`Metric`] from the MetricVec. In that case, the
    /// [`Metric`] will still exist, but it will not be exported anymore, even if a
    /// [`Metric`] with the same label values is created later. See also the
    /// CounterVec example.
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

    /// `get_metric_with` returns the [`Metric`] for the given Labels map (the
    /// label names must match those of the VariableLabels in Desc). If that label map is
    /// accessed for the first time, a new [`Metric`] is created. Implications of
    /// creating a [`Metric`] without using it and keeping the
    /// [`Metric`] for later use are the same as for GetMetricWithLabelValues.
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
    /// occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// use prometheus::{CounterVec, Opts};
    /// let vec = CounterVec::new(
    ///     Opts::new("requests_total", "Number of requests."),
    ///     &["code", "http_method"]
    /// ).unwrap();
    /// vec.with_label_values(&["404", "POST"]).inc()
    /// ```
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
    fn desc(&self) -> Vec<&Desc> {
        vec![&self.v.desc]
    }

    fn collect(&self) -> Vec<MetricFamily> {
        vec![self.v.collect()]
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::counter::CounterVec;
    use crate::gauge::GaugeVec;
    use crate::metrics::{Metric, Opts};

    #[test]
    fn test_counter_vec_with_labels() {
        let vec = CounterVec::new(
            Opts::new("test_couter_vec", "test counter vec help"),
            &["l1", "l2"],
        )
        .unwrap();

        let mut labels = HashMap::new();
        labels.insert("l1", "v1");
        labels.insert("l2", "v2");
        assert!(vec.remove(&labels).is_err());

        vec.with(&labels).inc();
        assert!(vec.remove(&labels).is_ok());
        assert!(vec.remove(&labels).is_err());

        let mut labels2 = HashMap::new();
        labels2.insert("l1", "v2");
        labels2.insert("l2", "v1");

        vec.with(&labels).inc();
        assert!(vec.remove(&labels2).is_err());

        vec.with(&labels).inc();

        let mut labels3 = HashMap::new();
        labels3.insert("l1", "v1");
        assert!(vec.remove(&labels3).is_err());
    }

    #[test]
    fn test_counter_vec_with_label_values() {
        let vec = CounterVec::new(
            Opts::new("test_vec", "test counter vec help"),
            &["l1", "l2"],
        )
        .unwrap();

        assert!(vec.remove_label_values(&["v1", "v2"]).is_err());
        vec.with_label_values(&["v1", "v2"]).inc();
        assert!(vec.remove_label_values(&["v1", "v2"]).is_ok());

        vec.with_label_values(&["v1", "v2"]).inc();
        assert!(vec.remove_label_values(&["v1"]).is_err());
        assert!(vec.remove_label_values(&["v1", "v3"]).is_err());
    }

    #[test]
    fn test_gauge_vec_with_labels() {
        let vec = GaugeVec::new(
            Opts::new("test_gauge_vec", "test gauge vec help"),
            &["l1", "l2"],
        )
        .unwrap();

        let mut labels = HashMap::new();
        labels.insert("l1", "v1");
        labels.insert("l2", "v2");
        assert!(vec.remove(&labels).is_err());

        vec.with(&labels).inc();
        vec.with(&labels).dec();
        vec.with(&labels).add(42.0);
        vec.with(&labels).sub(42.0);
        vec.with(&labels).set(42.0);

        assert!(vec.remove(&labels).is_ok());
        assert!(vec.remove(&labels).is_err());
    }

    #[test]
    fn test_gauge_vec_with_label_values() {
        let vec = GaugeVec::new(
            Opts::new("test_gauge_vec", "test gauge vec help"),
            &["l1", "l2"],
        )
        .unwrap();

        assert!(vec.remove_label_values(&["v1", "v2"]).is_err());
        vec.with_label_values(&["v1", "v2"]).inc();
        assert!(vec.remove_label_values(&["v1", "v2"]).is_ok());

        vec.with_label_values(&["v1", "v2"]).inc();
        vec.with_label_values(&["v1", "v2"]).dec();
        vec.with_label_values(&["v1", "v2"]).add(42.0);
        vec.with_label_values(&["v1", "v2"]).sub(42.0);
        vec.with_label_values(&["v1", "v2"]).set(42.0);

        assert!(vec.remove_label_values(&["v1"]).is_err());
        assert!(vec.remove_label_values(&["v1", "v3"]).is_err());
    }

    #[test]
    fn test_vec_get_metric_with() {
        let vec = CounterVec::new(
            Opts::new("test_vec", "test counter vec help"),
            &["b", "c", "a"],
        )
        .unwrap();

        // create a new metric that labels are {b" => "c", "c" => "a" "a" => "b"}.
        let mut labels = HashMap::new();
        labels.insert("a", "b");
        labels.insert("b", "c");
        labels.insert("c", "a");
        let c = vec.get_metric_with(&labels).unwrap();
        let m = c.metric();
        let label_pairs = m.label;
        assert_eq!(label_pairs.len(), labels.len());
        for lp in label_pairs.iter() {
            assert_eq!(
                lp.value.as_ref().unwrap(),
                labels[lp.name.as_ref().unwrap().as_str()]
            );
        }
    }
}
