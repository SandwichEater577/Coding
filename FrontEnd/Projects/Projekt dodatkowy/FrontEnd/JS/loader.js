const loadContent = () => {
  const root = document.getElementById("root");

  if (!root || root.dataset.loaded === "true") {
    return;
  }

  root.dataset.loaded = "true";

  root.innerHTML = `
    <div class="cards-container">
        <a href="/aviation" class="main-button_item">
            <picture>
                <source srcset="../../BackEnd/Assets/main_button_aircraft_en.webp" type="image/webp">
                <img src="../../BackEnd/Assets/main_button_aircraft_en.webp" alt="Aviation">
            </picture>
        </a>
        <a href="/helicopters" class="main-button_item">
            <picture>
                <source srcset="../../BackEnd/Assets/main_button_helicopter_en.webp" type="image/webp">
                <img src="../../BackEnd/Assets/main_button_helicopter_en.webp" alt="Helicopters">
            </picture>
        </a>
        <a href="/ground" class="main-button_item">
            <picture>
                <source srcset="../../BackEnd/Assets/main_button_tank_en.webp" type="image/webp">
                <img src="../../BackEnd/Assets/main_button_tank_en.webp" alt="Ground Vehicles">
            </picture>
        </a>
        <a href="/ships" class="main-button_item">
            <picture>
                <source srcset="../../BackEnd/Assets/main_button_ship_en.webp" type="image/webp">
                <img src="../../BackEnd/Assets/main_button_ship_en.webp" alt="Ships">
            </picture>
        </a>
        <a href="/boats" class="main-button_item">
            <picture>
                <source srcset="../../BackEnd/Assets/main_button_boat_en.webp" type="image/webp">
                <img src="../../BackEnd/Assets/main_button_boat_en.webp" alt="Boats">
            </picture>
        </a>
    </div>
  `;
};

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", loadContent);
} else {
  loadContent();
}
