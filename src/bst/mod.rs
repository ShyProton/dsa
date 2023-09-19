use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Node {
    Data {
        data: i32,
        count: i32,
        left: Box<Node>,
        right: Box<Node>,
    },
    Null,
}

impl Node {
    fn leaf(data: i32) -> Self {
        Self::Data {
            data,
            count: 1,
            left: Box::new(Self::Null),
            right: Box::new(Self::Null),
        }
    }

    pub fn print_nodes(&self) {
        if let Self::Data {
            data,
            count,
            left,
            right,
        } = self
        {
            left.print_nodes();
            println!("{count}: {data}");
            right.print_nodes();
        }
    }

    #[must_use]
    pub fn search(&self, value: i32) -> &Self {
        if let Self::Data {
            data,
            count: _,
            left,
            right,
        } = self
        {
            // The comparison between the current node's value and what we want to find will tell
            // us which branch to take. Larger values will be found more towards the right.
            match data.cmp(&value) {
                Ordering::Less => return right.search(value),
                Ordering::Equal => return self,
                Ordering::Greater => return left.search(value),
            };
        }

        // Once we reach a lone leaf (or find an equal node) we return.
        self
    }

    pub fn insert(&mut self, value: i32) {
        if let Self::Data {
            data,
            count,
            left,
            right,
        } = self
        {
            // The comparison between the current node's value and what we want to insert will tell
            // us which branch to take. Larger values will be inserted more towards the right.
            match (*data).cmp(&value) {
                Ordering::Less => right.insert(value),
                Ordering::Equal => *count += 1,
                Ordering::Greater => left.insert(value),
            }

            return;
        }

        // Once we find a lone leaf we give that leaf our inserted value.
        *self = Self::leaf(value);
    }

    #[must_use]
    pub fn sum(&self) -> i32 {
        if let Self::Data {
            data,
            count,
            left,
            right,
        } = self
        {
            return count * data + left.sum() + right.sum();
        }

        0
    }

    #[must_use]
    pub fn validate_tree(&self) -> bool {
        self.validate_subtree(i32::MIN, i32::MAX)
    }

    fn validate_subtree(&self, min: i32, max: i32) -> bool {
        if let Self::Data {
            data,
            count: _,
            left,
            right,
        } = self
        {
            if *data < min || *data > max {
                return false;
            }

            return left.validate_subtree(min, data - 1) && right.validate_subtree(data + 1, max);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn bst() {
        for _ in 0..100 {
            let mut root = Node::leaf(50);
            let mut rng = rand::thread_rng();

            for _ in 0..100_000 {
                root.insert(rng.gen_range(1..=100_000));
            }

            let meaning_of_life_branch = root.search(42);
            assert!(meaning_of_life_branch.validate_tree());
        }
    }
}
