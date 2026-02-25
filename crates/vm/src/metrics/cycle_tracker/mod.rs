use std::collections::BTreeMap;

/// Stats for a nested span in the execution segment that is tracked by the [`CycleTracker`].
#[derive(Clone, Debug, Default)]
pub struct SpanInfo {
    /// The name of the span.
    pub tag: String,
    /// The cycle count at which the span starts.
    pub start: usize,
    /// Maps (dsl_ir, opcode) to number of times opcode was executed
    pub counts: BTreeMap<(Option<String>, String), usize>,
}

#[derive(Clone, Debug)]
pub struct CycleTracker {
    /// Stack of span names, with most recent at the end
    stack: Vec<SpanInfo>,
    /// Depth of the stack.
    depth: usize,
    max_depth: usize,
}

impl Default for CycleTracker {
    fn default() -> Self {
        Self {
            stack: Vec::new(),
            depth: 0,
            max_depth: std::env::var("CYCLE_TRACKER_MAX_DEPTH")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2),
        }
    }
}

impl CycleTracker {
    pub fn new(max_depth: usize) -> Self {
        Self {
            max_depth,
            ..Default::default()
        }
    }

    pub fn top(&self) -> Option<&String> {
        match self.stack.last() {
            Some(span) => Some(&span.tag),
            _ => None,
        }
    }

    /// Starts a new cycle tracker span for the given name.
    /// If a span already exists for the given name, it ends the existing span and pushes a new one
    /// to the vec.
    pub fn start(&mut self, mut name: String, cycles_count: usize, num_insns_by_dsl: &BTreeMap<(Option<String>, String), usize>) {
        // hack to remove "CT-" prefix
        if name.starts_with("CT-") {
            name = name.split_off(3);
        }
        self.depth += 1;
        if self.depth > self.max_depth {
            return;
        }
        self.stack.push(SpanInfo {
            tag: name.clone(),
            start: cycles_count,
            counts: num_insns_by_dsl.clone(),
        });
        let padding = "│ ".repeat(self.depth);
        tracing::info!("{}┌╴{}", padding, name);
    }

    /// Ends the cycle tracker span for the given name.
    /// If no span exists for the given name, it panics.
    pub fn end(&mut self, mut name: String, cycles_count: usize, num_insns_by_dsl: &BTreeMap<(Option<String>, String), usize>) {
        // hack to remove "CT-" prefix
        if name.starts_with("CT-") {
            name = name.split_off(3);
        }
        // keep padding info before pop
        let padding = "│ ".repeat(self.depth);
        self.depth -= 1;
        if self.depth >= self.max_depth {
            return;
        }
        let SpanInfo { tag, start, counts: num_insns_start } = self.stack.pop().unwrap();
        assert_eq!(tag, name, "Stack top does not match name");
        let span_cycles = cycles_count - start;
        for (dsl_opcode, num_insns) in num_insns_by_dsl {
            let start_count = num_insns_start.get(dsl_opcode).cloned().unwrap_or(0);
            let span_count = num_insns - start_count;
            if span_count > 0 {
                tracing::info!("{}│   ({:?},{}): {} instructions", padding, dsl_opcode.0, dsl_opcode.1, span_count);
            }
        }
        tracing::info!("{}└╴({}) {} cycles, abs: {}", padding, name, span_cycles, cycles_count);
    }

    /// Ends the current cycle tracker span.
    pub fn force_end(&mut self) {
        self.stack.pop();
    }

    /// Get full name of span with all parent names separated by ";" in flamegraph format
    pub fn get_full_name(&self) -> String {
        self.stack
            .iter()
            .map(|span_info| span_info.tag.clone())
            .collect::<Vec<String>>()
            .join(";")
    }
}

#[cfg(feature = "metrics")]
mod emit {
    use metrics::counter;

    use super::CycleTracker;

    impl CycleTracker {
        pub fn increment_opcode(&self, (dsl_ir, opcode): &(Option<String>, String)) {
            let labels = [
                ("opcode", opcode.clone()),
                ("dsl_ir", dsl_ir.clone().unwrap_or_default()),
                ("cycle_tracker_span", self.get_full_name()),
            ];
            counter!("frequency", &labels).increment(1u64);
        }

        pub fn increment_cells_used(
            &self,
            (dsl_ir, opcode, air_name): &(Option<String>, String, String),
            trace_cells_used: usize,
        ) {
            if trace_cells_used == 0 {
                return;
            }
            let labels = [
                ("air_name", air_name.clone()),
                ("opcode", opcode.clone()),
                ("dsl_ir", dsl_ir.clone().unwrap_or_default()),
                ("cycle_tracker_span", self.get_full_name()),
            ];
            counter!("cells_used", &labels).increment(trace_cells_used as u64);
        }
    }
}
