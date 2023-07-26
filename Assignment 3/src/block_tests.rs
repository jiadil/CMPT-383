#[cfg(test)]
mod block_tests {
    use crate::block::Block;
    use std::fmt::Write;

    #[test]
    fn hashing_blocks_16() {
        let mut b0 = Block::initial(16);
        b0.set_proof(56231);
        assert_eq!(b0.hash_string(), "0000000000000000000000000000000000000000000000000000000000000000:0:16::56231");

        let mut b1 = Block::next(&b0, String::from("message"));
        b1.set_proof(2159);
        assert_eq!(b1.hash_string(), "6c71ff02a08a22309b7dbbcee45d291d4ce955caa32031c50d941e3e9dbd0000:1:16:message:2159");

        let mut output = String::new();
        // fmt::Write for String always returns Ok() and never Err.
        write!(&mut output, "{:02x}", b0.hash()).unwrap();
        assert_eq!(output, "6c71ff02a08a22309b7dbbcee45d291d4ce955caa32031c50d941e3e9dbd0000");
        output.clear();

        write!(&mut output, "{:02x}", b1.hash()).unwrap();
        assert_eq!(output, "9b4417b36afa6d31c728eed7abc14dd84468fdb055d8f3cbe308b0179df40000");
    }

    #[test]
    fn hashing_blocks_12() {
        let mut output = String::new();

        let mut b0 = Block::initial(12);
        b0.set_proof(52642);
        assert_eq!(b0.hash_string(), "0000000000000000000000000000000000000000000000000000000000000000:0:12::52642");

        let mut b1 = Block::next(&b0, String::from("this is a test"));
        b1.set_proof(52644);
        assert_eq!(b1.hash_string(), "a2cc0d303f229299519ac5dfd4b226bf64022e8d6cbdc0e5d462a7ecd8a768dc:1:12:this is a test:52644");

        write!(&mut output, "{:02x}", b0.hash()).unwrap();
        assert_eq!(output, "a2cc0d303f229299519ac5dfd4b226bf64022e8d6cbdc0e5d462a7ecd8a768dc");
        output.clear();

        write!(&mut output, "{:02x}", b1.hash()).unwrap();
        assert_eq!(output, "4f8897b90d211616e329efa9d5711a6124289fc5253119a10dbeaa349478a488");
    }

    #[test]
    fn valid_hashes_16() {
        let mut b0 = Block::initial(16);
        assert_eq!(b0.is_valid_for_proof(0), false);
        b0.set_proof(56231);
        assert_eq!(b0.is_valid_for_proof(56231), true);
        assert_eq!(b0.is_valid(), true);
    }

    #[test]
    fn valid_hashes_19() {
        let mut b1 = Block::initial(19);
        b1.set_proof(87745);
        assert_eq!(b1.is_valid_for_proof(87745), true);
        assert_eq!(b1.is_valid(), true);

        let mut b2 = Block::next(&b1, String::from("hash example 1234"));
        b2.set_proof(1407891);
        assert_eq!(b2.is_valid_for_proof(1407891), true);
        assert_eq!(b2.is_valid(), true);
        b2.set_proof(346082);
        assert_eq!(b2.is_valid_for_proof(346082), false);
        assert_eq!(b2.is_valid(), false);
    }

    #[test]
    fn valid_hashes_12() {
        let mut b0 = Block::initial(12);
        assert_eq!(b0.is_valid_for_proof(0), false);
        b0.set_proof(56231);
        assert_eq!(b0.is_valid_for_proof(56231), false);
        assert_eq!(b0.is_valid(), false);
    }

    #[test]
    fn mining_tasks_7() {
        let mut output = String::new();

        let mut b0 = Block::initial(7);
        b0.mine(10);
        assert_eq!(b0.hash_string(), "0000000000000000000000000000000000000000000000000000000000000000:0:7::385");
        write!(&mut output, "{:02x}", b0.hash()).unwrap();
        assert_eq!(output, "379bf2fb1a558872f09442a45e300e72f00f03f2c6f4dd29971f67ea4f3d5300");
        output.clear();

        let mut b1 = Block::next(&b0, String::from("this is an interesting message"));
        b1.mine(10);
        assert_eq!(b1.hash_string(), "379bf2fb1a558872f09442a45e300e72f00f03f2c6f4dd29971f67ea4f3d5300:1:7:this is an interesting message:20");
        write!(&mut output, "{:02x}", b1.hash()).unwrap();
        assert_eq!(output, "4a1c722d8021346fa2f440d7f0bbaa585e632f68fd20fed812fc944613b92500");
        output.clear();


        let mut b2 = Block::next(&b1, String::from("this is not interesting"));
        b2.mine(10);
        assert_eq!(b2.hash_string(), "4a1c722d8021346fa2f440d7f0bbaa585e632f68fd20fed812fc944613b92500:2:7:this is not interesting:40");
        write!(&mut output, "{:02x}", b2.hash()).unwrap();
        assert_eq!(output, "ba2f9bf0f9ec629db726f1a5fe7312eb76270459e3f5bfdc4e213df9e47cd380");
        output.clear();
    }

    #[test]
    fn mining_tests_20() {
        let mut output = String::new();

        let mut bb0 = Block::initial(20);
        bb0.mine(10);
        assert_eq!(bb0.hash_string(), "0000000000000000000000000000000000000000000000000000000000000000:0:20::1209938");
        write!(&mut output, "{:02x}", bb0.hash()).unwrap();
        assert_eq!(output, "19e2d3b3f0e2ebda3891979d76f957a5d51e1ba0b43f4296d8fb37c470600000");
        output.clear();

        let mut bb1 = Block::next(&bb0, String::from("this is an interesting message"));
        bb1.mine(10);
        assert_eq!(bb1.hash_string(), "19e2d3b3f0e2ebda3891979d76f957a5d51e1ba0b43f4296d8fb37c470600000:1:20:this is an interesting message:989099");
        write!(&mut output, "{:02x}", bb1.hash()).unwrap();
        assert_eq!(output, "a42b7e319ee2dee845f1eb842c31dac60a94c04432319638ec1b9f989d000000");
        output.clear();

        let mut bb2 = Block::next(&bb1, String::from("this is not interesting"));
        bb2.mine(10);
        assert_eq!(bb2.hash_string(), "a42b7e319ee2dee845f1eb842c31dac60a94c04432319638ec1b9f989d000000:2:20:this is not interesting:1017262");
        write!(&mut output, "{:02x}", bb2.hash()).unwrap();
        assert_eq!(output, "6c589f7a3d2df217fdb39cd969006bc8651a0a3251ffb50470cbc9a0e4d00000");
        output.clear();
    }

    #[test]
    fn mining_tests_12() {
        let mut output = String::new();

        let mut bb0 = Block::initial(12);
        bb0.mine(1);
        assert_eq!(bb0.hash_string(), "0000000000000000000000000000000000000000000000000000000000000000:0:12::100");
        write!(&mut output, "{:02x}", bb0.hash()).unwrap();
        assert_eq!(output, "fc4f1a87a88011e9e7dccdb9f1df7c9dca2e64b2b5c9f4c8b830e79c10903000");
        output.clear();

        let mut bb1 = Block::next(&bb0, String::from("this is a test"));
        bb1.mine(1);
        assert_eq!(bb1.hash_string(), "fc4f1a87a88011e9e7dccdb9f1df7c9dca2e64b2b5c9f4c8b830e79c10903000:1:12:this is a test:912");
        write!(&mut output, "{:02x}", bb1.hash()).unwrap();
        assert_eq!(output, "88f31b23e44045b8b3202b5bce621fe5b2e78b7748401dfed87b96e716596000");
        output.clear();

        let mut bb2 = Block::next(&bb1, String::from("this is another test"));
        bb2.mine(1);
        assert_eq!(bb2.hash_string(), "88f31b23e44045b8b3202b5bce621fe5b2e78b7748401dfed87b96e716596000:2:12:this is another test:7487");
        write!(&mut output, "{:02x}", bb2.hash()).unwrap();
        assert_eq!(output, "064d64aaf2359475868047f6b5c3fd46802c7256c4e3aa1d6683e9572d4eb000");
        output.clear();
    }
}
