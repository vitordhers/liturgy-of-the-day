use enums::{LiturgicalColor, Month};

struct FixedLiturgyDay {
    day: i32,
    month: Month,
    name: &'static str,
    color: LiturgicalColor,
}

enum CelebrationType {
    None,
    OptionalCommemoration,
    OptionalMemorial,
    Memorial,
    Feast,
    Solemnity,
}

impl FixedLiturgyDay {
    fn new(day: i32, month: Month, name: &'static str, color: LiturgicalColor) -> Self {
        Self {
            day,
            month,
            name,
            color,
        }
    }
}

pub const LITURGY_FIXED_DAYS: &'static [FixedLiturgyDay] = &[
    FixedLiturgyDay::new(
        1,
        Month::January,
        "Solemnity of The Blessed Virgin Mary, the Holy Mother of God", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        6,
        Month::January,
        "Epiphany (Theophany) of Our Lord and Savior Jesus Christ", // All glory and honour is yours, for ever and ever
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        25,
        Month::January,
        "Feast of the Conversion of Saint Paul, the Apostle", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        28,
        Month::January,
        "Feast of Saint Thomas Aquinas, Doctor of the Church", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        22,
        Month::February,
        "Feast of the Chair of Saint Peter", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        19,
        Month::March,
        "Solemnity of Saint Joseph, Husband of Mary", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        21,
        Month::April,
        "Feast of Saint Mark, Evangelist", // Pray for us
        LiturgicalColor::Red,
    ),
    FixedLiturgyDay::new(
        1,
        Month::May,
        "Feast of Saint Joseph, the Worker", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        8,
        Month::May,
        "Feast of Saint Michael, the Archangel", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        31,
        Month::May,
        "The Visitation of the Blessed Virgin Mary", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        24,
        Month::June,
        "The Nativity of St. John, the Baptist", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        29,
        Month::June,
        "Feast of Saints Peter and Paul, Apostles", // Pray for us
        LiturgicalColor::Red,
    ),
    FixedLiturgyDay::new(
        25,
        Month::July,
        "Feast of Saint James the Greater, Apostle", // Pray for us
        LiturgicalColor::Red,
    ),
    FixedLiturgyDay::new(
        5,
        Month::August,
        "Feast of Saint Mary Major", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        15,
        Month::August,
        "The Assumption of the Blessed Virgin Mary", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        22,
        Month::August,
        "The Queenship of Mary", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        24,
        Month::August,
        "Feast of Saint Bartholomew, Apostle", // Pray for us
        LiturgicalColor::Red,
    ),
    FixedLiturgyDay::new(
        28,
        Month::August,
        "Feast of Saint Augustine, Doctor of the Church", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        29,
        Month::August,
        "Passion of Saint John, The Baptist", // Pray for us
        LiturgicalColor::Red,
    ),
    FixedLiturgyDay::new(
        8,
        Month::September,
        "The Nativity of the Blessed Virgin Mary", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        12,
        Month::September,
        "Most Holy Name of the Blessed Virgin Mary / Our Lady of Aparecida", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        15,
        Month::September,
        "Our Lady of Sorrows", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        21,
        Month::September,
        "Feast of Saint Matthew, Apostle", // Pray for us
        LiturgicalColor::Red,
    ),
    FixedLiturgyDay::new(
        29,
        Month::September,
        "Feast of Saint Michael, Saint Gabriel, and Saint Raphael", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        2,
        Month::October,
        "Feast of the Guardian Angels", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        7,
        Month::October,
        "Our Lady of the Rosary", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        15,
        Month::October,
        "Feast of Saint Teresa of Avila, Doctor of the Church", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        15,
        Month::October,
        "Feast of Saint Luke, Evangelist", // Pray for us
        LiturgicalColor::Red,
    ),
    FixedLiturgyDay::new(
        28,
        Month::October,
        "Feast of Saint Simon and Saint Jude Thaddeus, Apostles", // Pray for us
        LiturgicalColor::Red,
    ),
    FixedLiturgyDay::new(
        1,
        Month::November,
        "Universal Feast of All Saints", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(2, Month::November, "All Soul's Day", LiturgicalColor::Black),
    FixedLiturgyDay::new(
        21,
        Month::November,
        "The Presentation of the Blessed Virgin Mary", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        30,
        Month::November,
        "Feast of Saint Andrew, Apostle", // Pray for us
        LiturgicalColor::Red,
    ),
    FixedLiturgyDay::new(
        8,
        Month::December,
        "Solemnity of the Immaculate Conception of the Blessed Virgin Mary", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        21,
        Month::December,
        "Feast of Saint Thomas, Apostle", // Pray for us
        LiturgicalColor::Red,
    ),
    FixedLiturgyDay::new(
        25,
        Month::December,
        "Our Lord and Savior Jesus Christ's birthday", // All glory and honour is yours, for ever and ever
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(
        27,
        Month::December,
        "Feast of Saint John, Apostle and Evangelist", // Pray for us
        LiturgicalColor::Gold,
    ),
    FixedLiturgyDay::new(31, Month::December, "New Year's Eve", LiturgicalColor::Gold),
];

/*
//// Fixed Days

// Black
// 02/11 - All Soul's Day

// Red
// 29/08 - Passion of Saint John The Baptist
/// Feasts of Martyrs, Apostles, and Evangelists
// Feasts of the Apostles:
// 30/11 - Feast of Saint Andrew, Apostle - November 30th
// 21/12 - Feast of Saint Thomas, Apostle - December 21st
// 29/06 - Feast of Saints Peter and Paul, Apostles - June 29th
// 25/07 - Feast of Saint James the Greater, Apostle - July 25th
// 24/08 - Feast of Saint Bartholomew, Apostle - August 24th
// 21/09 - Feast of Saint Matthew, Apostle - September 21st
// 28/10 - Feast of Saint Simon and Saint Jude, Apostles - October 28th
// Feasts of the Evangelists:
// 21/04 - Feast of Saint Mark, Evangelist - April 25th
// 21/09 - Feast of Saint Matthew, Evangelist - September 21st
// 18/10 - Feast of Saint Luke, Evangelist - October 18th
// 27/12 - Feast of Saint John, Apostle and Evangelist - December 27th -> Ask the Holy Church

// White / Gold
/// 24-12~1stSunday after 06/01 Christmastide (from [Vigil] of Christmas to the Baptism of the Lord)

/// Feasts of Our Lord other than those of His Passion
// 25/12 - Christmas
// 31/12 - New Year's Eve
// 01/01 - Solemnity of Mary, the Holy Mother of God  (Marian feast days)
// 06/01 - Epiphany (Theophany) of Our Lord and Savior Jesus Christ

/// 27/12 - Feast of Saint John, Apostle and Evangelist
/// 22/01 - Feast of the Chair of Saint Peter
/// 25/01 - Feast of the Conversion of Saint Paul, the Apostle
/// 24/06 - The Nativity of St. John the Baptist


/// Marian feast days
// 01/01 - Solemnity of Mary, Mother of God
// 31/05 - (in some locations July 2): The Visitation of the Blessed Virgin Mary
// 05/08 - Saint Mary Major (Santa Maria Maggiore; also known as Saint Mary of the Snows) August 5
// 15/08 - The Assumption of the Blessed Virgin Mary (Solemnity) August 15
// 22/08 - The Queenship of Mary: August 22
// 08/09 - The Nativity of the Blessed Virgin Mary also known as Marymas: September 8
// 12/09 - Most Holy Name of the Blessed Virgin Mary: September 12
// 15/09 - Our Lady of Sorrows: September 15
// 07/10 - Our Lady of the Rosary: October 7
// 21/11 - The Presentation of Mary: November 21
// 08/12 - Solemnity of the Immaculate Conception of the Blessed Virgin Mary: December 8


/// Feasts of the Angels
// 29/09 - Feast of Saint Michael, Saint Gabriel, and Saint Raphael - September 29th
// 02/10 - Feast of the Guardian Angels - October 2nd
// 08/05 - Feast of Saint Michael the Archangel - May 8th

/// Feasts of non-martyred saints or confessors
/// Feast of All Saints
// 01/11 - Universal Feast of All Saints - November 1st
// Local Feast Days - These feast days vary depending on the region, diocese, or religious order that venerates the saint
/// Feasts of Doctors of the Church
// 28/08 - Saint Augustine (August 28th)
// 28/01 - Saint Thomas Aquinas (January 28th)
// 15/10 - Saint Teresa of Avila (October 15th)

/// Other feasts of Saint Joseph
// 19/03 - Solemnity of Saint Joseph, Husband of Mary
// 01/05 - Feast of Saint Joseph the Worker


/// Holy Thursday
/// Easter season (from the Easter Vigil up to the Vigil of Pentecost) Easter time is the period of 50 days, spanning from Easter Sunday to Pentecost Sunday.[ Normae Universales de Anno Liturgico et de Calendario (NUALC), 22]


/// Sacrament of Baptism
/// Sacrament of Matrimony
/// Sacrament of Holy Orders
*/
