use ex1;

#[test]
    fn it_adds1(){
        let mut v = Vector::new();

        v.add(3);
        v.add(2);

        assert!(v.tabel[0]<v.tabel[1]);

    }