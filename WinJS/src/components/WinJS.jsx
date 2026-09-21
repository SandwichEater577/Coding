export default function WinJS({
  userPowerActionMoreVisible,
  setUserPowerActionMoreVisible,
}) {
  return (
    <>
      <div id="winjs-left-section">
        <div></div>
        <div></div>
      </div>
      <div id="winjs-right-section">
        <div id="user-profile-picture">
          <div>
            <img
              src="https://avatars.githubusercontent.com/u/236650016?v=4&size=64"
              alt=""
            />
          </div>
        </div>
        <div id="user-actions">
          <div id="top-user-actions">
            <button className="user-action-button">Administator</button>
            <button className="user-action-button">Documents</button>
            <button className="user-action-button">Pictures</button>
            <button className="user-action-button">Music</button>
          </div>
          <div id="middle-user-actions">
            <button className="user-action-button">Games</button>
            <button className="user-action-button">Computer</button>
          </div>
          <div id="bottom-user-actions">
            <button className="user-action-button">Control Panel</button>
            <button className="user-action-button">
              Devices &amp; Printers
            </button>
            <button className="user-action-button">Default Programs</button>
            <button className="user-action-button">Help &amp; Support</button>
          </div>
        </div>
        <div id="user-power-actions">
          <div>
            <button className="user-power-action-button" id="shut-down-button">
              Shut Down
            </button>
            <div id="user-power-actions-more">
              {userPowerActionMoreVisible && (
                <div id="revealed-user-power-actions">
                  <button>Switch User</button>
                  <button>Log Off</button>
                  <button>Restart</button>
                </div>
              )}
              {!userPowerActionMoreVisible ? (
                <button
                  className="user-power-action-button"
                  id="reveal-user-power-actions-more-button"
                  onClick={() => setUserPowerActionMoreVisible(true)}
                >
                  ⌄
                </button>
              ) : (
                <button
                  className="user-power-action-button"
                  id="hide-user-power-actions-more-button"
                  onClick={() => setUserPowerActionMoreVisible(false)}
                >
                  ⌃
                </button>
              )}
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
