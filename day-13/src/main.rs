use std::{cmp::Ordering, fs, io, str::FromStr};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let result_1 = part_1(&input);
    println!("day-13/part-1: {result_1}");

    let result_2 = part_2(&input);
    println!("day-13/part-2: {result_2}");
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum PacketElement {
    Int(u32),
    Nested(Packet),
}

impl PartialOrd for PacketElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketElement {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketElement::Int(x), PacketElement::Int(y)) => x.cmp(y),
            (PacketElement::Nested(px), PacketElement::Nested(py)) => px.cmp(py),
            (n @ PacketElement::Nested(_), i @ PacketElement::Int(_)) => i.cmp(n).reverse(),
            (PacketElement::Int(x), n @ PacketElement::Nested(_)) => {
                let mut packet = Packet::new();
                packet.push(PacketElement::Int(*x));
                PacketElement::Nested(packet).cmp(n)
            }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
struct Packet {
    elements: Vec<PacketElement>,
}

impl Packet {
    fn new() -> Packet {
        Packet {
            elements: Vec::new(),
        }
    }

    fn push(&mut self, element: PacketElement) {
        self.elements.push(element);
    }
}

struct PacketScanner {
    data: Vec<char>,
    cursor: usize,
}

impl PacketScanner {
    fn peek_next_char(&self) -> char {
        self.data[self.cursor]
    }

    fn next_char(&mut self) -> char {
        let c = self.peek_next_char();
        self.cursor += 1;
        c
    }

    fn next_int(&mut self) -> PacketElement {
        let mut int_str = String::new();
        while self.peek_next_char().is_numeric() {
            int_str.push(self.next_char());
        }
        let n = int_str
            .parse::<u32>()
            .expect("All chars in range have been checked to be numeric");
        PacketElement::Int(n)
    }

    fn next_nested(&mut self) -> PacketElement {
        let packet = self.parse_packet();
        PacketElement::Nested(packet)
    }

    fn parse_packet(&mut self) -> Packet {
        let mut packet = Packet::new();
        let c = self.next_char();
        assert!(c == '[');
        let mut done = false;
        while !done {
            match self.peek_next_char() {
                ']' => {
                    self.next_char();
                    done = true;
                }

                ',' => {
                    self.next_char();
                }

                '[' => {
                    packet.push(self.next_nested());
                }

                c if c.is_numeric() => {
                    packet.push(self.next_int());
                }

                _ => unreachable!(),
            }
        }
        packet
    }
}

impl FromIterator<char> for PacketScanner {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> PacketScanner {
        PacketScanner {
            data: iter.into_iter().collect(),
            cursor: 0,
        }
    }
}

#[derive(Debug)]
struct ParsePacketError;

impl FromStr for Packet {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Packet, ParsePacketError> {
        let mut scanner = s.chars().collect::<PacketScanner>();
        let packet = scanner.parse_packet();
        Ok(packet)
    }
}

fn part_1(s: &str) -> u32 {
    s.lines()
        .filter_map(|line| (!line.is_empty()).then(|| Packet::from_str(line).unwrap()))
        .collect::<Vec<_>>()
        .chunks(2)
        .zip(1..)
        .filter_map(|(packet, n)| (packet[0] < packet[1]).then_some(n))
        .sum()
}

fn part_2(input: &str) -> u32 {
    let start_sentinal = Packet::from_str("[[2]]").unwrap();
    let end_sentinal = Packet::from_str("[[6]]").unwrap();
    let mut packets = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Packet::from_str)
        .map(|r| r.expect("All input lines should represent valid packets"))
        .collect::<Vec<_>>();
    packets.push(start_sentinal.clone());
    packets.push(end_sentinal.clone());
    packets.sort();

    let start_index = packets
        .iter()
        .position(|p| p == &start_sentinal)
        .expect("Start sentinal should have been pushed into the packets");

    let end_index = packets
        .iter()
        .position(|p| p == &end_sentinal)
        .expect("End sentinal should have been pushed into the packets");

    ((start_index + 1) * (end_index + 1)) as u32
}

#[cfg(test)]
mod day_13_tests {
    use crate::{Packet, PacketElement};

    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Packet {
        let mut c = Packet::new();

        for i in iter {
            c.push(PacketElement::Int(i));
        }

        c
    }

    mod packet_construction {
        use super::*;
        #[test]
        fn a_new_packet_is_empty() {
            let packet = Packet::new();
            assert!(packet.elements.is_empty());
        }

        #[test]
        fn a_packet_can_contain_an_element() {
            let mut packet = Packet::new();
            packet.push(PacketElement::Int(3));
            assert!(!packet.elements.is_empty());
        }

        #[test]
        fn a_packet_can_contain_another_packet() {
            let mut packet = Packet::new();
            packet.push(PacketElement::Nested(Packet::new()));
            assert!(!packet.elements.is_empty());
        }
    }

    mod packet_element_equality {
        use super::*;

        #[test]
        fn two_packet_ints_are_equal_if_contained_ints_are_equal() {
            assert_eq!(PacketElement::Int(42), PacketElement::Int(42));
        }

        #[test]
        fn two_packet_ints_are_not_equal_if_contained_ints_are_not_equal() {
            assert_ne!(PacketElement::Int(0), PacketElement::Int(42));
        }

        #[test]
        fn two_packet_nested_are_eq_id_nested_packets_are_equal() {
            let mut inner_1 = Packet::new();
            inner_1.push(PacketElement::Int(42));
            let mut outer_1 = Packet::new();
            outer_1.push(PacketElement::Nested(inner_1));

            let mut inner_2 = Packet::new();
            inner_2.push(PacketElement::Int(42));
            let mut outer_2 = Packet::new();
            outer_2.push(PacketElement::Nested(inner_2));

            assert_eq!(outer_1, outer_2);
        }

        #[test]
        fn two_packet_nested_are_not_eq_id_nested_packets_are_not_equal() {
            let mut inner_1 = Packet::new();
            inner_1.push(PacketElement::Int(42));
            let mut outer_1 = Packet::new();
            outer_1.push(PacketElement::Nested(inner_1));

            let mut inner_2 = Packet::new();
            inner_2.push(PacketElement::Int(0));
            let mut outer_2 = Packet::new();
            outer_2.push(PacketElement::Nested(inner_2));

            assert_ne!(outer_1, outer_2);
        }
    }

    mod packet_element_ordering {
        use super::*;

        #[test]
        fn if_both_values_are_integers_the_smaller_integer_is_less_than_the_larger_integer() {
            assert!(PacketElement::Int(1) > PacketElement::Int(0));
        }

        #[test]
        fn if_both_values_are_lists_then_the_lists_are_compared_elementwise() {
            let mut inner_1 = Packet::new();
            inner_1.push(PacketElement::Int(0));
            inner_1.push(PacketElement::Int(10000));
            let outer_1 = PacketElement::Nested(inner_1);

            let mut inner_2 = Packet::new();
            inner_2.push(PacketElement::Int(42));
            let outer_2 = PacketElement::Nested(inner_2);

            assert!(outer_2 > outer_1)
        }

        #[test]
        fn if_both_lists_are_pairwise_eq_then_shorter_list_is_pairwise_first() {
            let mut inner_1 = Packet::new();
            inner_1.push(PacketElement::Int(23));
            inner_1.push(PacketElement::Int(96));
            inner_1.push(PacketElement::Int(0));
            let outer_1 = PacketElement::Nested(inner_1);

            let mut inner_2 = Packet::new();
            inner_2.push(PacketElement::Int(23));
            inner_2.push(PacketElement::Int(96));
            inner_2.push(PacketElement::Int(0));
            inner_2.push(PacketElement::Int(42));
            let outer_2 = PacketElement::Nested(inner_2);

            assert!(outer_2 > outer_1)
        }

        #[test]
        fn comparing_an_int_that_is_less_than_a_list() {
            let i = PacketElement::Int(23);

            let inner = from_iter([34, 96, 0, 42]);
            let outer = PacketElement::Nested(inner);

            assert!(i < outer);
        }

        #[test]
        fn comparing_an_int_that_is_greater_than_a_list() {
            let i = PacketElement::Int(1000);

            let inner = from_iter([34, 96, 0, 42]);
            let outer = PacketElement::Nested(inner);

            assert!(i > outer);
        }

        #[test]
        fn comparing_a_list_that_is_less_than_an_int() {
            let i = PacketElement::Int(1000);

            let inner = from_iter([34, 96, 0, 42]);
            let outer = PacketElement::Nested(inner);

            assert!(outer < i);
        }

        mod examples {
            use super::*;
            use crate::{Packet, PacketElement};

            // Compare [1,1,3,1,1] vs [1,1,5,1,1]
            #[test]
            fn pair_1() {
                let left = from_iter([1, 1, 3, 1, 1]);
                let right = from_iter([1, 1, 5, 1, 1]);
                assert!(left < right);
            }

            // Compare [[1],[2,3,4]] vs [[1],4]
            #[test]
            fn pair_2() {
                let mut left = Packet::new();
                let inner_1 = from_iter([1]);
                left.push(PacketElement::Nested(inner_1));
                let inner_2 = from_iter([2, 3, 4]);
                left.push(PacketElement::Nested(inner_2));

                let mut right = Packet::new();
                let inner = from_iter([1]);
                right.push(PacketElement::Nested(inner));
                right.push(PacketElement::Int(4));

                assert!(left < right);
            }

            // Compare [9] vs [[8,7,6]]
            #[test]
            fn pair_3() {
                let left = from_iter([9]);

                let mut right = Packet::new();
                let right_inner = from_iter([8, 7, 6]);
                right.push(PacketElement::Nested(right_inner));

                assert!(left > right);
            }

            // Compare [[4,4],4,4] vs [[4,4],4,4,4]
            #[test]
            fn pair_4() {
                let mut left = Packet::new();
                let inner_left = from_iter([4, 4]);
                left.push(PacketElement::Nested(inner_left));
                left.push(PacketElement::Int(4));
                left.push(PacketElement::Int(4));

                let mut right = Packet::new();
                let inner_right = from_iter([4, 4]);
                right.push(PacketElement::Nested(inner_right));
                right.push(PacketElement::Int(4));
                right.push(PacketElement::Int(4));
                right.push(PacketElement::Int(4));

                assert!(left < right);
            }

            //Compare [7,7,7,7] vs [7,7,7]
            #[test]
            fn pair_5() {
                let left = from_iter([7, 7, 7, 7]);
                let right = from_iter([7, 7, 7]);
                assert!(left > right);
            }

            // Compare [] vs [3]
            #[test]
            fn pair_6() {
                let left = Packet::new();

                let right = from_iter([3]);
                assert!(left < right);
            }

            // Compare [[[]]] vs [[]]
            #[test]
            fn pair_7() {
                let mut left = Packet::new();
                let mut outer_inner = Packet::new();
                let inner_inner = Packet::new();
                outer_inner.push(PacketElement::Nested(inner_inner));
                left.push(PacketElement::Nested(outer_inner));

                let mut right = Packet::new();
                let inner = Packet::new();
                right.push(PacketElement::Nested(inner));

                assert!(left > right);
            }

            // Compare [1,[2,[3,[4,[5,6,7]]]],8,9] vs [1,[2,[3,[4,[5,6,0]]]],8,9]
            #[test]
            fn pair_8() {
                let mut left = Packet::new();
                let inner_4_left = from_iter([5, 6, 7]);
                let mut inner_3_left = Packet::new();
                inner_3_left.push(PacketElement::Int(4));
                inner_3_left.push(PacketElement::Nested(inner_4_left));
                let mut inner_2_left = Packet::new();
                inner_2_left.push(PacketElement::Int(3));
                inner_2_left.push(PacketElement::Nested(inner_3_left));
                let mut inner_1_left = Packet::new();
                inner_1_left.push(PacketElement::Int(2));
                inner_1_left.push(PacketElement::Nested(inner_2_left));
                left.push(PacketElement::Int(1));
                left.push(PacketElement::Nested(inner_1_left));
                left.push(PacketElement::Int(8));
                left.push(PacketElement::Int(9));

                let mut right = Packet::new();
                let inner_4_right = from_iter([5, 6, 0]);
                let mut inner_3_right = Packet::new();
                inner_3_right.push(PacketElement::Int(4));
                inner_3_right.push(PacketElement::Nested(inner_4_right));
                let mut inner_2_right = Packet::new();
                inner_2_right.push(PacketElement::Int(3));
                inner_2_right.push(PacketElement::Nested(inner_3_right));
                let mut inner_1_right = Packet::new();
                inner_1_right.push(PacketElement::Int(2));
                inner_1_right.push(PacketElement::Nested(inner_2_right));
                right.push(PacketElement::Int(1));
                right.push(PacketElement::Nested(inner_1_right));
                right.push(PacketElement::Int(8));
                right.push(PacketElement::Int(9));

                assert!(left > right);
            }
        }
    }

    mod packet_parsing {
        use super::*;
        use crate::{Packet, PacketElement, ParsePacketError};
        use std::str::FromStr;

        #[test]
        fn an_empty_packet() -> Result<(), ParsePacketError> {
            let s = "[]";
            let p = Packet::from_str(s)?;
            assert_eq!(p, Packet::new());
            Ok(())
        }

        #[test]
        fn a_packet_with_an_int() -> Result<(), ParsePacketError> {
            let s = "[42]";
            let p = Packet::from_str(s)?;
            let expected = from_iter([42]);
            assert_eq!(p, expected);
            Ok(())
        }

        #[test]
        fn a_packet_with_a_list_of_ints() -> Result<(), ParsePacketError> {
            let s = "[1,1,3,1,1]";
            let p = Packet::from_str(s)?;
            let expected = from_iter([1, 1, 3, 1, 1]);
            assert_eq!(p, expected);
            Ok(())
        }

        #[test]
        fn a_packet_with_a_nested_packet() -> Result<(), ParsePacketError> {
            let s = "[[]]";
            let p = Packet::from_str(s)?;
            let mut expected = Packet::new();
            let inner = Packet::new();
            expected.push(PacketElement::Nested(inner));
            assert_eq!(p, expected);
            Ok(())
        }

        #[test]
        fn a_nested_pattern() -> Result<(), ParsePacketError> {
            let s = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
            let p = Packet::from_str(s)?;

            let mut expected = Packet::new();
            let inner_4_expected = from_iter([5, 6, 7]);
            let mut inner_3_expected = Packet::new();
            inner_3_expected.push(PacketElement::Int(4));
            inner_3_expected.push(PacketElement::Nested(inner_4_expected));
            let mut inner_2_expected = Packet::new();
            inner_2_expected.push(PacketElement::Int(3));
            inner_2_expected.push(PacketElement::Nested(inner_3_expected));
            let mut inner_1_expected = Packet::new();
            inner_1_expected.push(PacketElement::Int(2));
            inner_1_expected.push(PacketElement::Nested(inner_2_expected));
            expected.push(PacketElement::Int(1));
            expected.push(PacketElement::Nested(inner_1_expected));
            expected.push(PacketElement::Int(8));
            expected.push(PacketElement::Int(9));

            assert_eq!(p, expected);
            Ok(())
        }
    }
}
