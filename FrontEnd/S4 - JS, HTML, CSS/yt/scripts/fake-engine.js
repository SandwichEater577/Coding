Object.defineProperty(Array.prototype, "shuffle", {
  value: function () {
    for (let i = this.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [this[i], this[j]] = [this[j], this[i]];
    }
    return this;
  },
});

const recommendations = [
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/_oO4Qi5aVZs/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLDjd8Fd1kdPCMa3SaM7ayppw8Ol2Q",
    time: "2:17:36",
    title:
      "Build and Deploy a Fully Responsive Website with Modern UI/UX in React JS with Tailwind",
    channel: "JavaScript Mastery",
    views: "130 tys. wyświetleń",
    date: "3 dni temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/N3AkSS5hXMA/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLAorjseM_6Xb7Loy9ErIJK2plULMw",
    time: "4:33",
    title: "What Is React (React js) & Why Is It So Popular?",
    channel: "Programming with Mosh",
    views: "624 tys. wyświetelń",
    date: "4 lata temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/cuHDQhDhvPE/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLBULOIh9q2RvPhmVZ97JgvpuYiBVA",
    time: "21:58",
    title: "I built the same app 10 times // Which JS Framework is best?",
    channel: "Fireship",
    views: "1,4 mln wyświetleń",
    date: "1 rok temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/uVwtVBpw7RQ/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLDskYFDfPHACylxh-m5ynveBKw-ww",
    time: "21:58",
    title: "What is Node js?",
    channel: "Programming with Mosh",
    views: "593 tys. wyświetleń",
    date: "4 lata temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/udxqsJXJM5Q/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLBBQGzw24eGp9t9poin2Xz_1a0uQw",
    time: "57:34",
    title: "Podstawy programowania w JavaScript w 60 MINUT",
    channel: "Jak nauczyć się programowania",
    views: "169 tys. wyświetleń",
    date: "2 lata temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/w5tS0TKyCvA/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCedaXfAdQQdKViQy9A6txhUS5hDg",
    time: "25:40",
    title: "Szybki kurs Visual Studio Code",
    channel: "MMC School",
    views: "20 tys. wyświetleń",
    date: "10 miesięcy temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/RJEKMbD_kEk/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLAKiHgPgEFfkbB-I4MQKXh5D5wEvw",
    time: "35:16",
    title:
      "Kurs CSS odc.1: Kaskadowe arkusze stylów - pierwszy projekt, wiedza podstawowa",
    channel: "Pasja Informatyki",
    views: "708 tys. wyświetleń",
    date: "7 lat temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/opNgrPv3Qw8/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLD4Aet8jW7aUAp9ol3tyljP6rGnzw",
    time: "2:20:11",
    title: "Podstawy HTML + CSS w dwie godziny - kurs krok po kroku!",
    channel: "Jak zacząć programować?",
    views: "113 tys. wyświetleń",
    date: "1 rok temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/ArTVfdHOB-M/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLDWL0V6Soeo07m1hp3LblmSU08cWg",
    time: "14:26",
    title:
      "Magic Navigation Menu Indicator using Html CSS & Javascript | Curve Outside Effects",
    channel: "Online Tutorials",
    views: "1,1 mln. wyświetleń",
    date: "8 miesięcy temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/uDjv-CsXqbs/hqdefault.jpg?sqp=-oaymwEcCOADEI4CSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCbNk8Pww8wJxWCPMFLqHimP0GbFA",
    time: "14:26",
    title: "position: absolute wyjaśnione w 10 minut *poradnik CSS*",
    channel: "Jak zacząć programować?",
    views: "4,9 tys. wyświetleń",
    date: "1 rok temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/CG__N4SS1Fc/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLBi98gxuTQYjuKoKSuKaPf9Z5piWg",
    time: "11:51",
    title: "Front-end dev reacts to amazing CSS-only Codepens",
    channel: "Kevin Powell",
    views: "1,1 mln. wyświetleń",
    date: "1 rok temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/q0KJEmMlG20/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLADuuJuEVD-_aKzPUa1dfURbXyo5w",
    time: "12:47",
    title: "Wszystko o CSS Flexbox w 12 minut | Poradnik o CSS",
    channel: "MikuCode",
    views: "18 tys. wyświetleń",
    date: "2 lata temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/k2IydkL3EOs/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCcwEEFfkJngu-OEKn2Ar15UsXWjg",
    time: "39:22",
    title:
      "Kurs HTML odc.1: Tworzenie stron www. Pierwszy projekt, wiedza podstawowa",
    channel: "Pasja Informatyki",
    views: "1,9 mln. wyświetleń",
    date: "7 lat temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/Qhaz36TZG5Y/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLDbnmNVR6BnSuv5Tqh0qQSN25BrcA",
    time: "9:39",
    title: "10 CSS Pro Tips - Code this, NOT that!",
    channel: "Fireship",
    views: "1,2 mln. wyświetleń",
    date: "1 rok temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/o_GnXwio5Hs/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCJzT0sczzwPhnDE7_PVNixotdg5w",
    time: "36:47",
    title: "Flexbox - praktyczny tutorial CSS dla początkujących!",
    channel: "Jak zacząć programować?",
    views: "6,7 tys. wyświetleń",
    date: "6 miesięcy temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/Q7AOvWpIVHU/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLA2Yp7BYeJxAhJO2J8bxZYjUkx3vg",
    time: "15:38",
    title:
      "Build a Mindblowing 3D Portfolio Website // Three.js Beginner’s Tutorial",
    channel: "Fireship",
    views: "1,4 mln. wyświetleń",
    date: "1 rok temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/H60ByFADAfs/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLDFCsvCUNiX6imtHU9j42NujqNZgw",
    time: "13:05",
    title: "Three.js personal portfolios are amazing...",
    channel: "Filip",
    views: "198 tys. wyświetleń",
    date: "4 miesiące temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/BOF79TAIkYQ/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLBA9QmqMCSM5h8CCjuXn-mnUkKeNw",
    time: "1:28:49",
    title:
      "Responsive Admin Dashboard Using HTML CSS & JavaScript with Light & Dark Mode",
    channel: "EGATOR",
    views: "541 tys. wyświetleń",
    date: "9 miesiący temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/-QgJgZCJvo4/hqdefault.jpg?sqp=-oaymwEcCOADEI4CSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLDc9wGHrZuCz1eudvoHN4kwz9jNgA",
    time: "37:45",
    title: "I Challenged The CSS King To A CSS Battle",
    channel: "Web Dev Simplified",
    views: "1,1 mln. wyświetleń",
    date: "1 rok temu",
  },
  {
    thumbnailUrl:
      "https://i.ytimg.com/vi/6VjvT5uMm-w/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLDTqNHBHC4dWQ8Lm3qYCloPqCu5GQ",
    time: "10:23",
    title: "1 Design Trend in 2022 That I Think will be BIG! (7 Examples)",
    channel: "DesignCourse",
    views: "362 tys. wyświetleń",
    date: "7 miesięcy temu",
  },
];

//https://i.ytimg.com/vi/uVwtVBpw7RQ/hq720.jpg?sqp=-oaymwEcCNAFEJQDSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLDskYFDfPHACylxh-m5ynveBKw-ww

const mapToRecommendation = (props) => {
  return `<div class="recommendation">
          <div class="thumbnail">
            <img
              src="${props.thumbnailUrl}"
            />
            <div class="time">
              <span>${props.time}</span>
            </div>
          </div>

          <div class="metadata">
            <h4 class="title">${props.title}</h4>
            <span class="channel text gray small">${props.channel}</span>
            <div>
              <span class="views text gray small">${props.views}</span>
              <span class="date text gray small">${props.date}</span>
            </div>
          </div>
        </div>`;
};

const renderRecommendations = () => {
  const recommendationsElement = document.getElementById("recommendations");
  recommendationsElement.innerHTML = recommendations
    .shuffle()
    .map(mapToRecommendation)
    .join("");
};

window.onload = () => {
  renderRecommendations();
};
