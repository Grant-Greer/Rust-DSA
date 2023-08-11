
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(reverse("Think different.".to_string()), ".tnereffid knihT".to_string());
    }

    #[test]
    fn test2() {
        assert_eq!(reverse("It doesnt make sense to hire smart people and tell them what to do; we hire smart people so they can tell us what to do.".to_string()), ".od ot tahw su llet nac yeht os elpoep trams erih ew ;od ot tahw meht llet dna elpoep trams erih ot esnes ekam tnseod tI".to_string());
    }

    #[test]
    fn test3() {
        assert_eq!(reverse("Innovation is the ability to see change as an opportunity - not a threat".to_string()), "taerht a ton - ytinutroppo na sa egnahc ees ot ytiliba eht si noitavonnI".to_string());
    }

    #[test]
    fn test4() {
        assert_eq!(reverse("Everything is based on a simple rule: Quality is the best business plan.".to_string()), ".nalp ssenisub tseb eht si ytilauQ :elur elpmis a no desab si gnihtyrevE".to_string());
    }

    #[test]
    fn test5() {
        assert_eq!(reverse("The people who are crazy enough to think they can change the world are the ones who do.".to_string()), ".od ohw seno eht era dlrow eht egnahc nac yeht kniht ot hguone yzarc era ohw elpoep ehT".to_string());
    }

    #[test]
    fn test6() {
        assert_eq!(reverse("Dont let the noise of others opinions drown out your own inner voice.".to_string()), ".eciov renni nwo ruoy tuo nword snoinipo srehto fo esion eht tel tnoD".to_string());
    }

    #[test]
    fn test7() {
        assert_eq!(reverse("Learn continually; Theres always one more thing to learn.".to_string()), ".nrael ot gniht erom eno syawla serehT ;yllaunitnoc nraeL".to_string());
    }

    #[test]
    fn test8() {
        assert_eq!(reverse("Quality is more important than quantity. One home run is much better than two doubles.".to_string()), ".selbuod owt naht retteb hcum si nur emoh enO .ytitnauq naht tnatropmi erom si ytilauQ".to_string());
    }

    #[test]
    fn test9() {
        assert_eq!(reverse("Your time is limited so dont waste it living someone elses life.".to_string()), ".efil sesle enoemos gnivil ti etsaw tnod os detimil si emit ruoY".to_string());
    }

    #[test]
    fn test10() {
        assert_eq!(reverse("The only way to be truly satisfied is to do what you believe is great work.".to_string()), ".krow taerg si eveileb uoy tahw od ot si deifsitas ylurt eb ot yaw ylno ehT".to_string());
    }