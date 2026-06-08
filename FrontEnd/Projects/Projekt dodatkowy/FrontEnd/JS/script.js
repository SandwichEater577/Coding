const groundVehiclesButtonOnTheChooseVehicleTypeScreen = document.getElementById("ground-vehicles-button-on-the-choose-vehicle-type-screen");
const helicopterButtonOnThechooseVehicleTypeScreen = document.getElementById("helicopters-button-on-the-choose-vehicle-type-screen");
const aviationButtonOnThechooseVehicleTypeScreen = document.getElementById("aviation-button-on-the-choose-vehicle-type-screen");
const shipsButtonOnTheChooseVehicleTypeScreen = document.getElementById("ships-button-on-the-choose-vehicle-type-screen");
const boatsButtonOnTheChooseVehicleTypeScreen = document.getElementById("boats-button-on-the-choose-vehicle-type-screen");
const chooseVehicleTypeScreen = document.getElementById("choose-vehicle-type-screen");
const root = document.getElementById("main-root");

const loadContent = () => {

  if (!root || root.dataset.loaded === "true") {
    return;
  }

  root.dataset.loaded = "true";

  chooseVehicleTypeScreen.classList.remove("hidden");
};

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", loadContent);
} else {
  loadContent();
}

const attachChooseVehicleTypeListeners = () => {
  if (!chooseVehicleTypeScreen || !chooseVehicleTypeScreen.classList.contains("screen-active")) return;

  if (aviationButtonOnThechooseVehicleTypeScreen) {
    aviationButtonOnThechooseVehicleTypeScreen.addEventListener("click", (e) => {
      e.preventDefault();
    });
  }

  if (helicopterButtonOnThechooseVehicleTypeScreen) {
    helicopterButtonOnThechooseVehicleTypeScreen.addEventListener("click", (e) => {
      e.preventDefault();
    });
  }

  if (groundVehiclesButtonOnTheChooseVehicleTypeScreen) {
    groundVehiclesButtonOnTheChooseVehicleTypeScreen.addEventListener("click", (e) => {
      e.preventDefault();
    });
  }

  if (shipsButtonOnTheChooseVehicleTypeScreen) {
    shipsButtonOnTheChooseVehicleTypeScreen.addEventListener("click", (e) => {
      e.preventDefault();
    });
  }

  if (boatsButtonOnTheChooseVehicleTypeScreen) {
    boatsButtonOnTheChooseVehicleTypeScreen.addEventListener("click", (e) => {
      e.preventDefault();
    });
  }
};

// attach after DOM ready / after content is loaded
if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", attachChooseVehicleTypeListeners);
} else {
  attachChooseVehicleTypeListeners();
}

// choose vehicle type screen
// |
// |---> ground vehicles
//    |---> choose Vehicle.ground
// |---> helicopters
//    |---> choose Vehicle.helicopter
// |---> aviation
//    |---> choose Vehicle.aviation
// |---> ships
//    |---> choose Vehicle.ships
// |---> boats
//    |---> choose Vehicle.boats