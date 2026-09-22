import "./Desktop.css";
import "./WindowCSS/WinJS.css";
import WinJS from "./WinJS.jsx";
import { useEffect } from "react";

export default function Desktop({
  openWindows,
  setOpenWindows,
  userPowerActionMoreVisible,
  setUserPowerActionMoreVisible,
}) {
  function handleClickOutsideWindow(event) {
    const target = event.target;

    if (!(target instanceof Element)) {
      return;
    }

    const clickedInsideWinJS =
      target.closest(".WinJS-window") ||
      target.closest(".start-button") ||
      target.closest("[aria-label='Start']") ||
      target.closest("#winjs-left-section") ||
      target.closest("#winjs-right-section") ||
      target.closest("#user-power-actions-more") ||
      target.closest(".user-action-button") ||
      target.closest(".user-power-action-button") ||
      target.closest("[class*='WinJS']") ||
      target.closest("[id*='WinJS']") ||
      target.closest("[data-window='WinJS']") ||
      target.closest("[data-title-name='WinJS']") ||
      target.closest("[title='WinJS']") ||
      target.closest("[aria-label='WinJS']");

    if (clickedInsideWinJS) {
      return;
    }

    setOpenWindows((currentOpenWindows) =>
      currentOpenWindows.filter((window) => window["title-name"] !== "WinJS"),
    );
    setUserPowerActionMoreVisible(false);
  }

  useEffect(() => {
    document.addEventListener("mousedown", handleClickOutsideWindow);

    return () => {
      document.removeEventListener("mousedown", handleClickOutsideWindow);
    };
  }, [setOpenWindows, setUserPowerActionMoreVisible]);

  return (
    <>
      <div id="desktop-container">
        <img
          src="https://imgs.search.brave.com/o6nJV0OtfqFLeVQjgV2wsU9CIzY_u63jCgQ97rvTYQg/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9jZG4u/d2FsbHBhcGVyc2Fm/YXJpLmNvbS82Ny81/OS80b2JoSEIuanBn"
          alt=""
        />
        <div id="desktop-content">
          {openWindows.map((window) => (
            <div
              id={`window-${window["unique-id"]}`}
              className={`window  ${window["title-name"]}-window`}
              key={window["unique-id"]}
              style={
                window["title-name"] === "WinJS"
                  ? null
                  : { width: window["width"], height: window["height"] }
              }
            >
              {window["title-name"] === "WinJS" ? (
                <WinJS
                  userPowerActionMoreVisible={userPowerActionMoreVisible}
                  setUserPowerActionMoreVisible={setUserPowerActionMoreVisible}
                />
              ) : window["title-name"] === "Calculator" ? (
                <div className="calculator-window-content"></div>
              ) : null}
            </div>
          ))}
        </div>
      </div>
    </>
  );
}
