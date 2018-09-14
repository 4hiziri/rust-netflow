lazy_static! {
    static ref NUM_ID: HashSet<u16> = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set.insert(6);
        set.insert(7);
        set.insert(10);
        set.insert(13);
        set.insert(14);
        set.insert(16);
        set.insert(17);
        set.insert(19);
        set.insert(20);
        set.insert(21);
        set.insert(22);
        set.insert(23);
        set.insert(24);
        set.insert(25);
        set.insert(26);
        set.insert(29);
        set.insert(30);
        set.insert(31);
        set.insert(32);
        set.insert(33);
        set.insert(34);
        set.insert(35);
        set.insert(36);
        set.insert(37);
        set.insert(38);
        set.insert(39);
        set.insert(40);
        set.insert(41);
        set.insert(42);
        set.insert(43);
        set.insert(46);
        set.insert(47);
        set.insert(48);
        set.insert(49);
        set.insert(50);
        set.insert(51);
        set.insert(52);
        set.insert(53);
        set.insert(54);
        set.insert(55);
        set.insert(58);
        set.insert(59);
        set.insert(60);
        set.insert(61);
        set.insert(64);
        set.insert(65);
        set.insert(66);
        set.insert(67);
        set.insert(68);
        set.insert(69);
        set.insert(70);
        set.insert(71);
        set.insert(72);
        set.insert(73);
        set.insert(74);
        set.insert(75);
        set.insert(76);
        set.insert(77);
        set.insert(78);
        set.insert(79);
        set.insert(85);
        set.insert(86);
        set.insert(87);
        set.insert(88);
        set.insert(89);
        set.insert(91);
        set.insert(92);
        set.insert(93);
        set.insert(94);
        set.insert(96);
        set.insert(98);
        set.insert(99);
        set.insert(100);
        set.insert(102);
        set.insert(103);
        set.insert(104);

        set
    };
    static ref BYTES_ID: HashSet<u16> = {
        let mut set = HashSet::new();
        set.insert(90);

        set
    };
    static ref IPV4_ID: HashSet<u16> = {
        let mut set = HashSet::new();
        set.insert(8);
        set.insert(12);
        set.insert(15);
        set.insert(18);
        set.insert(44);
        set.insert(45);

        set
    };
    static ref IPV6_ID: HashSet<u16> = {
        let mut set = HashSet::new();
        set.insert(27);
        set.insert(28);
        set.insert(62);
        set.insert(63);

        set
    };
    static ref MACADDR_ID: HashSet<u16> = {
        let mut set = HashSet::new();
        set.insert(56);
        set.insert(57);
        set.insert(80);
        set.insert(81);

        set
    };
    static ref STRING_ID: HashSet<u16> = {
        let mut set = HashSet::new();
        set.insert(82);
        set.insert(83);
        set.insert(84);

        set
    };
    static ref BITS_ID: HashSet<u16> = {
        let mut set = HashSet::new();
        set.insert(95);

        set
    };
}
