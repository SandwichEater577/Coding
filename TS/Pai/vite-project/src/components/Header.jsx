import "./Header.css";

export default function Header() {
  return (
    <>
      <header id="main-header-container">
        <div className="main-header-section" id="main-header-left">
          <div id="main-header-logo">PhonkHub</div>
          <div id="main-header-div-for-buttons">
            <button id="main-header-home-button">Home</button>
            <button id="main-header-library-button">Library</button>
          </div>
        </div>
        <div className="main-header-section" id="main-header-middle">
          <input
            id="main-header-search"
            type="text"
            placeholder="Search..."
          ></input>
        </div>
        <div className="main-header-section" id="main-header-right">
          <button id="main-header-profile-button">
            <div id="main-header-profile-picture">ProfilePic</div>
            <img
              src="./../assets/arrow-down.png"
              id="main-header-profile-arrow"
            ></img>
          </button>
        </div>
      </header>
    </>
  );
}
