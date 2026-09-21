import "./DesktopTaskbar.css";

export default function DesktopTaskbar({
  isAtDesktop,
  taskbarItems,
  setOpenWindows,
  openWindows,
}) {
  function handleTaskbarItemClick(itemName) {
    setOpenWindows((currentOpenWindows) => {
      const winJSAlreadyOpen = currentOpenWindows.some(
        (window) => window["title-name"] === "WinJS",
      );

      if (itemName === "WinJS" && winJSAlreadyOpen) {
        return currentOpenWindows.filter(
          (window) => window["title-name"] !== "WinJS",
        );
      }

      return [
        ...currentOpenWindows,
        {
          "unique-id": `${itemName}-${Date.now()}`,
          "x-pos": 0,
          "y-pos": 0,
          width: 400,
          height: 400,
          "title-name": itemName,
          taskbarItemReference: itemName,
        },
      ];
    });
  }

  const appTaskbarItems = taskbarItems.filter((item) => item.name !== "WinJS");

  return (
    <>
      {isAtDesktop && (
        <div id="taskbar-container">
          <div id="taskbar-content">
            <button
              type="button"
              className={`start-button${
                openWindows.some((window) => window["title-name"] === "WinJS")
                  ? " is-open"
                  : ""
              }`}
              aria-label="Start"
              onClick={() => handleTaskbarItemClick("WinJS")}
            />
            {appTaskbarItems.map((element) => {
              return (
                <button
                  key={element.name}
                  className="taskbar-item"
                  onClick={() => handleTaskbarItemClick(element.name)}
                >
                  <img
                    src={element.icon}
                    alt={element.name}
                    id={`${element.name}-icon-at-taskbar`}
                  ></img>
                </button>
              );
            })}
          </div>
        </div>
      )}
    </>
  );
}
