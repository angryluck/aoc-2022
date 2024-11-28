// LEFT SHOULD BE SMALLEST!

enum PacketPart {
    LeftBracket,
    RightBracket,
    Value(i32),
}

struct Packet(Vec<PacketPart>);

pub fn do_part1(input: &str) -> u32 {
    let packets = parse_input(input);
    // let packets = packets.into_iter().zip(1..);
    packets
        .into_iter()
        .zip(1..)
        .filter(|(packet_pair, _index)| left_packet_is_smaller(packet_pair))
        .map(|(_packet_pair, index)| index)
        .sum()
    // todo!()
}

fn parse_input(input: &str) -> Vec<(Vec<PacketPart>, Vec<PacketPart>)> {
    todo!()
}

fn left_packet_is_smaller(
    (left_packet, right_packet): &(Vec<PacketPart>, Vec<PacketPart>),
) -> bool {
    let mut left_packet = left_packet.iter();
    let mut right_packet = right_packet.iter();
    while let Some(left_val) = left_packet.next() {
        if let Some(right_val) = right_packet.next() {
            // Compare values:
            return true;
        } else {
            return false;
        }
    }
    true
    // todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    // use anyhow::Result;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9";

    #[test]
    fn part1_works() {
        let result = do_part1(INPUT);
        assert_eq!(result, 13)
    }
    //     }
}
