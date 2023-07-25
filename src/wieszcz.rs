//use axum::response::IntoResponse;
use rand::Rng;
//use clap::error::ContextValue::String;

pub fn title() -> String {
  let titles = ["Zagłada",
    "To już koniec",
    "Świat ginie",
    "Z wizytą w piekle",
    "Kataklizm",
    "Dzień z życia...",
    "Masakra",
    "Katastrofa",
    "Wszyscy zginiemy...",
    "Pokój?",
    "Koniec",
    "Koniec ludzkości",
    "Telefon do Boga",
    "Wieczne ciemności",
    "Mrok",
    "Mrok w środku dnia",
    "Ciemność",
    "Piorunem w łeb",
    "Marsz troli",
    "Szyderstwa Złego",
    "Okrponości świata",
    "Umrzeć po raz ostatni",
    "Potępienie",
    "Ból mózgu",
    "Wieczne wymioty",
    "Zatrute dusze",
    "Uciekaj",
    "Apokalipsa",
    "Złudzenie pryska",
    "Makabra",
    "Zagłada świata",
    "Śmierć",
    "Spokój",
    ];
  let mut rng = rand::thread_rng();
  let x = rng.gen_range(0..titles.len());
  format!("{}", titles[x])
}

pub fn go(strophe: usize) -> String {
    let mut rng = rand::thread_rng();
    let data0 = ["Czy na te zbrodnie nie będzie kary?",
        "Opustoszały bagna, moczary",
        "Na nic się modły zdadzą ni czary",
        "Z krwi mordowanych sączą puchary",
        "To nietoperze, węże, kalmary",
        "Próżno nieszczęśni sypią talary",
        "Za co nam znosić takie ciężary",
        "Złowrogo iskrzą kóbr okulary",
        "Próżno swe modły wznosi wikary",  //new
        "Pustoszą sny twoje złe nocne mary", //new
        "Próżno nieszczęśnik sypie talary", //grych
        "Przedziwnie tka się życia logarytm", //grych
        "Już Strach wypuścił swoje ogary", //grych
        "Niebawem zginiesz w szponach poczwary", //grych
        "Wbijają pale złote kafary", //grych
        "Życie odkrywa swoje przywary", //grych
        "Na dnie ponurej, pustej pieczary", //grych
        "Apokalipsy nadeszły czary", //frk
        "Upadły anioł wspomina chwałę", //frk
        "Życie ukrywa swoje przywary", //grych LAME but used
        "Dziwnych owadów wzlatują chmary", //new 201505
        "Bombowce biorą nasze namiary", //201505 restored
        "Nie da się chwycić z czartem za bary", //201505 restored
        "Próżno frajerzy sypią talary",
        "Nie da sie wyrwać czartom towaru",
        "Po co nam sączyć podłe browary",
        "Diler już nie dostarczy towaru",
        "Lokomotywa nie ma już pary",
        "Gdy nie każdego stać na browary",
        "Pożarł Hilary swe okulary",
        "Spowiły nas trujące opary",
        "To nie jest całka ani logarytm", // len() = 31
    ];
    let data1 = ["Już na arenę krew tryska",
        "Już piana cieknie im z pyska",
        "Już hen w oddali gdzieś błyska",
        "Śmierć w kącie czai się bliska",
        "Niesamowite duchów igrzyska",
        "Już zaciskając łapiska",
        "Zamiast pozostać w zamczyskach",
        "Rzeka wylewa z łożyska",
        "Nieszczęść wylała się miska", //new
        "Już zaciskając zębiska", //my
        "Otwarta nieszczęść walizka", //grych
        "Niczym na rzymskich boiskach", //grych
        "Czart wznieca swe paleniska", //my
        "A w mroku świecą zębiska", //grych - fix
        "Zewsząd dochodzą wyzwiska", //grych
        "Świętych głód wiary przyciska", //my
        "Ponuro patrzy z ich pyska", //grych
        "Mgła stoi na uroczyskach", //frk
        "Kości pogrzebią urwiska", //frk
        "Głód wiary tak nas przyciska", //grych - BAD - fixed
        "Runęły skalne zwaliska",
        "Czart rozpala paleniska", //grych - BAD fixed 201505
        "A w mroku świecą zębiska", //grych - BAD fixed
        "Znów pusta żebraka miska",
        "Diabelskie to są igrzyska",
        "Nie powiedz diabłu nazwiska",
        "Najgłośniej słychać wyzwiska",
        "Diabelskie mają nazwiska",
        "Tam uciekają ludziska",
        "Tak rzecze stara hipiska",
        "Gdzie dawne ludzi siedliska",
        "Najgłośniej piszczy hipiska",
    ];
    let data2 = ["Rwą pazurami swoje ofiary",
        "Nic nie pomoże tu druid stary",
        "To nocne zjawy i senne mary",
        "Niegroźne przy nich lwowskie batiary",
        "Pod wodzą księżnej diablic Tamary",
        "Z dala straszliwe trąbia fanfary",
        "Skąd ich przywiodły piekła bezmiary",
        "Zaś dookoła łuny, pożary",
        "A twoje ciało rozszarpie Wilk Szary", //new
        "Tu nie pomoże już siła wiary", //my
        "Tak cudzych nieszczęść piją nektary", //grych
        "Wszystko zalewa wrzący liparyt", //grych
        "Zabójcze są ich niecne zamiary", //my
        "Zatrute dusze łączą się w pary", //grych
        "Świat pokazuje swoje wymiary", //grych
        "Z życiem się teraz weźmiesz za bary", //my
        "Brak uczuć, chęci, czasem brak wiary", //grych
        "Wspomnij, co mówił Mickiewicz stary", //frk
        "Spalonych lasów straszą hektary", //frk
        "Z życiem się dzisiaj weźmiesz za bary", //grych - BAD - fixed
        "Ksiądz pozostaje nagle bez wiary", //jn 201505 new
        "Papież zaczyna odprawiać czary", //jn 201505 new
        "Tu nie pomoże paciorek, stary", //jn 201505 new
        "Niegroźne przy nich nawet Atari",
        "Takie są oto piekła bezmiary",
        "A teraz nagle jesteś już stary",
        "Mordercy liczą swoje ofiary",
        "I bez wartości są już dolary",
        "Gdzie się podziały te nenufary",
        "Upada oto dąb ten prastary",
        "Bystro śmigają nawet niezdary",
        "Już nieruchome ich awatary",
    ];
    let data3 = ["Wnet na nas też przyjdzie kryska",
      "Znikąd żadnego schroniska",
      "Powietrze tnie świst biczyska",
      "Rodem z czarciego urwiska",
      "I swąd nieznośny się wciska",
      "Huk, jak z wielkiego lotniska",
      "Złowroga brzmią ich nazwiska",
      "W kącie nieśmiało ktoś piska",
      "Ktoś obok morduje liska", //new
      "Krwią ociekają zębiska", //my
      "Wookoło dzikie piarżyska", //grych
      "I żądza czai się niska", //grych
      "Diabeł cię dzisiaj wyzyska", //grych
      "Płoną zagłady ogniska", //grych
      "Gwałt niech się gwałtem odciska!", //grych
      "Stoisz na skraju urwiska", //my
      "Tam szatan czarta wyiska", //grych
      "Uciekaj - przyszłość jest mglista", //frk
      "Nadziei złudzenie pryska", //frk
      "Wydziobią oczy ptaszyska", //grych - BAD fixed
      "Padają łby na klepisko", //new 201505 - restored
      "Śmierć zbiera żniwo w kołyskach", //new 201505 - restored
      "Coś znowu zgrzyta w łożyskach", //jn new 201505
      "Spadasz z wielkiego urwiska",
      "Lawa spod ziemi wytryska",
      "Wokoło grzmi albo błyska",
      "Fałszywe złoto połyska",
      "Najwięcej czart tu uzyska",
      "Owieczki Zły tu pozyska",
      "Owieczki spadły z urwiska",
      "Snują się dymy z ogniska",
      "To czarne lecą ptaszyska",
    ];
    let strophe: usize = if strophe > 10 { 4 } else { strophe };
    let strophe: usize = if strophe < 1 { 1 } else { strophe };
    let mut str = String::new();
    for _i in 0..strophe {
      let (x, y, z, q) = (
        rng.gen_range(0..data0.len()),
        rng.gen_range(0..data1.len()),
        rng.gen_range(0..data2.len()),
        rng.gen_range(0..data3.len()),
      );
      //log::info!("xs = {}", xs.len());
      str = format!("{}\n\n{}\n{}\n{}\n{}", str, data0[x], data1[y], data2[z], data3[q]);
    }
    str
}
