import { useState } from "react";
import Main from "./components/Main";

export default function App() {
  const [techniCoinCount, setTechniCoinCount] = useState(0);
  const [isShopNavOpen, setIsShopNavOpen] = useState(false);
  const [gainPerClick, setGainPerClick] = useState(1);

  return (
    <>
      <Main
        gainPerClick={gainPerClick}
        setGainPerClick={setGainPerClick}
        techniCoinCount={techniCoinCount}
        setTechniCoinCount={setTechniCoinCount}
        isShopNavOpen={isShopNavOpen}
        setIsShopNavOpen={setIsShopNavOpen}
      />
    </>
  );
}
