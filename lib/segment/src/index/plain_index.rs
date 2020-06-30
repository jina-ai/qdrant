
use crate::vector_storage::vector_storage::{VectorMatcher, ScoredPoint, VectorCounter};
use crate::index::index::{Index, PayloadIndex};
use crate::types::{Filter, PointOffsetType, ScoreType};
use crate::payload_storage::payload_storage::{ConditionChecker};


pub struct PlainPayloadIndex<'s> {
    condition_checker: Box<&'s dyn ConditionChecker>,
    vector_counter: Box<&'s dyn VectorCounter>
}

impl PayloadIndex for PlainPayloadIndex<'_> {
    fn estimate_cardinality(&self, query: &Filter) -> (usize, usize) {
        let mut matched_points = 0;
        for i in 0..self.vector_counter.vector_count() {
            if self.condition_checker.check(i, query) {
                matched_points += 1;
            }
        }
        (matched_points, matched_points)
    }

    fn query_points(&self, query: &Filter) -> Vec<usize> {
        let mut matched_points = vec![];
        for i in 0..self.vector_counter.vector_count() {
            if self.condition_checker.check(i, query) {
                matched_points.push(i);
            }
        }
        return matched_points
    }
}


pub struct PlainIndex<'s, El> {
    vector_matcher: Box<&'s dyn VectorMatcher<El>>,
    payload_index: Box<&'s dyn PayloadIndex>,
}

impl<'s, El> PlainIndex<'s, El> {
    fn new(
        vector_matcher: &'s dyn VectorMatcher<El>,
        condition_filter: &'s dyn PayloadIndex,
    ) -> PlainIndex<'s, El> {
        return PlainIndex {
            vector_matcher: Box::new(vector_matcher),
            payload_index: Box::new(condition_filter),
        };
    }
}


impl<'s, El> Index<El> for PlainIndex<'s, El> {
    fn search(&self, vector: &Vec<El>, filter: Option<&Filter>, top: usize) -> Vec<(PointOffsetType, ScoreType)> {
        match filter {
            Some(filter) => {
                let filtered_ids = self.payload_index.query_points(filter);
                self.vector_matcher.score_points(vector, &filtered_ids, 0)
            }
            None => self.vector_matcher.score_all(vector, top)
        }.iter().map(ScoredPoint::to_tuple).collect()
    }
}