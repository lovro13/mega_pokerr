use crate::logic::combinations::*;
use super::card::Card;
use super::hand_ranking::HandRanking;

// treba rangirati karte
pub fn best_combination(cards: &mut Vec<Card>) -> HandRanking {
    // vrne naj po vrsti urejene karte ki niso bile uporabljene v kombinaciji,
    // oziroma prva ali prvi dve naj bosta te dve ki poveste kako visoka je bila kombinaicija
    // TODO: urediti nums da bodo primerni za primerjanje
    assert!(
        cards.len() == 5,
        "Na mizi ni 5 kart, ko hočemo določiti zmagovalca (best_combination)"
    );
    let mut nums = Vec::new();
    for card in cards.iter() {
        nums.push(card.number.clone());
    }
    nums.sort();

    if is_royal_flush(cards) {
        return HandRanking::RoyalFlush(vec![]);
    } else if is_straight_flush(cards) {
        nums.reverse();
        let output = vec![nums.remove(0)];
        return HandRanking::StraightFlush(output);
    } else if is_four_of_a_kind(cards) {
        // pomoje da ni preveč wasteful če kloniram enume, bilo bi bolj robustno če bi delal z numsi brez da kloniram
        let mut high_card = Vec::new();
        if nums[0] == nums[1] {
            high_card.push(nums[0].clone());
            high_card.push(nums[4].clone());
        } else {
            high_card.push(nums[4].clone());
            high_card.push(nums[0].clone());
        }
        return HandRanking::FourOfAKind(high_card);
    } else if is_full_house(cards) {
        let mut high_card = Vec::new();
        if nums[0] == nums[1] && nums[1] == nums[2] {
            // torej tris na prvem mestu
            high_card.push(nums[0].clone());
            high_card.push(nums[4].clone());
        } else {
            // torej tris na zadnjem mestu
            high_card.push(nums[4].clone());
            high_card.push(nums[0].clone());
        }
        return HandRanking::FullHouse(high_card);
    } else if is_flush(cards) {
        nums.sort();
        return HandRanking::Flush(vec![nums.remove(4)]); // pomembna samo najvišja karta
    } else if is_straight(cards) {
        let mut nums = Vec::new();
        for card in cards.iter() {
            nums.push(card.number.clone());
        }
        nums.sort();
        return HandRanking::Straight(vec![nums.remove(4)]); // pomembna samo najvišja karta
    } else if is_three_of_a_kind(cards) {
        let mut high_card = Vec::new();

        if nums[0] == nums[1] && nums[1] == nums[2] {
            high_card.push(nums[0].clone());
            high_card.push(nums[4].clone());
            high_card.push(nums[3].clone());
        } else if nums[1] == nums[2] && nums[2] == nums[3] {
            high_card.push(nums[1].clone());
            high_card.push(nums[4].clone());
            high_card.push(nums[0].clone());
        } else if nums[2] == nums[3] && nums[3] == nums[4] {
            high_card.push(nums[2].clone());
            high_card.push(nums[1].clone());
            high_card.push(nums[0].clone());
        }
        assert!(
            high_card.len() == 3,
            "Nekaj je narobe, ko računamo ThreeOfAKind (best_combination)"
        );
        return HandRanking::ThreeOfAKind(high_card);
    } else if is_two_pair(cards) {
        let mut nums = Vec::new();
        for card in cards.iter() {
            nums.push(card.number.clone());
        }
        nums.sort();
        nums.reverse();
        let mut nums_clone = Vec::new();
        let mut high_card = Vec::new();
        for num in nums.iter() {
            if nums_clone.contains(num) {
                high_card.push(num.clone());
                nums_clone.retain(|x| x != num);
            } else {
                nums_clone.push(num.clone());
            }
        }

        nums_clone.sort();
        nums_clone.reverse();
        high_card.append(&mut nums_clone);
        return HandRanking::TwoPair(high_card);
    } else if is_one_pair(cards) {
        let mut nums = Vec::new();
        for card in cards.iter() {
            nums.push(card.number.clone());
        }
        nums.sort();
        nums.reverse();
        let mut nums_clone = Vec::new();
        let mut high_card = Vec::new();
        for num in nums.iter() {
            if nums_clone.contains(num) {
                high_card.push(num.clone());
                nums_clone.retain(|x| x != num);
            } else {
                nums_clone.push(num.clone());
            }
        }


        nums_clone.sort();
        nums_clone.reverse();
        high_card.append(&mut nums_clone);
        return HandRanking::OnePair(high_card);
    } else {
        nums.reverse();
        return HandRanking::HighCard(nums);
    }
}
