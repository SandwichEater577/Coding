import "./Main.css";
import { shopNavContent } from "../Data/LayoutData.js";

export default function Main({
  gainPerClick,
  setGainPerClick,
  setTechniCoinCount,
  techniCoinCount,
  isShopNavOpen,
  setIsShopNavOpen,
}) {
  const coinSvg = (
    <svg
      width="18"
      height="18"
      viewBox="0 0 24 24"
      role="img"
      aria-label="Coin"
      xmlns="http://www.w3.org/2000/svg"
    >
      <circle
        cx="12"
        cy="12"
        r="9"
        fill="#f5c542"
        stroke="#b8860b"
        strokeWidth="2"
      />
      <path
        d="M14.5 8.5c-.6-.5-1.4-.8-2.5-.8-1.4 0-2.3.7-2.3 1.7 0 2.4 4.6 1.1 4.6 3.5 0 1-.9 1.7-2.4 1.7-1.1 0-2-.3-2.7-.9M12 6.5v11"
        fill="none"
        stroke="#8a6100"
        strokeWidth="1.4"
        strokeLinecap="round"
      />
    </svg>
  );

  function handleUpgradeBougth(id) {
    switch (id) {
      case 1:
        if (!(techniCoinCount >= 100)) {
          return;
        }
        setGainPerClick(gainPerClick + 1);
        setTechniCoinCount(techniCoinCount - 100);
        break;
    }
  }

  return (
    <>
      <div id="main-container">
        <div id="click-count-header">
          <div id="click-count-text">Techni Coins: {techniCoinCount}</div>
          <button
            id="shop-nav-toggle"
            type="button"
            aria-label="Menu"
            aria-expanded={isShopNavOpen}
            aria-controls="shop-nav"
            onClick={() => setIsShopNavOpen(!isShopNavOpen)}
          >
            ☰
          </button>
        </div>

        <div id="click-area-container">
          <div id="click-area" style={{ userSelect: "none" }}>
            <button
              id="click-button"
              onClick={() => setTechniCoinCount(techniCoinCount + gainPerClick)}
            >
              <img id="svg-img" src="/favicon.svg" alt="favicon" />
            </button>
          </div>
          {isShopNavOpen && (
            <nav id="shop-nav">
              {shopNavContent.map((element) => (
                <button
                  onClick={() => handleUpgradeBougth(element.id)}
                  key={element.text}
                  type="button"
                >
                  {coinSvg} {element.price} - {element.text}
                </button>
              ))}
            </nav>
          )}
        </div>
      </div>
    </>
  );
}
