import React from "react";
import DesktopTaskbar from "./components/DesktopTaskbar";
import Desktop from "./components/Desktop";

export default function App() {
  const [isAtDesktop, setIsAtDesktop] = React.useState(true);
  const [userPowerActionMoreVisible, setUserPowerActionMoreVisible] =
    React.useState(false);
  const [openWindows, setOpenWindows] = React.useState([]);
  const [taskbarItems, setTaskbarItems] = React.useState([
    {
      icon: "https://imgs.search.brave.com/uz5vPBeYaOi-FWKmP8xiyaPiiPlh2ZY4sehoiqyzFFg/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly93d3cu/bG9nby53aW5lL2Ev/bG9nby9XaW5kb3dz/XzcvV2luZG93c183/LUxvZ28ud2luZS5z/dmc",
      name: "WinJS",
    },
    {
      icon: "https://imgs.search.brave.com/RKk98QvKVHs10rD52b-IPcZLYF4TgQtpkhsqXVsPYcc/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9zdGF0/aWMud2lraWEubm9j/b29raWUubmV0L2xv/Z29wZWRpYS9pbWFn/ZXMvMy8zMi9DYWxj/dWxhdG9yX1dpblhQ/LnN2Zy9yZXZpc2lv/bi9sYXRlc3Qvc2Nh/bGUtdG8td2lkdGgt/ZG93bi8yMDA_Y2I9/MjAyMjEyMjkxNDI4/Mzc",
      name: "Calculator",
    },
  ]);

  return (
    <>
      <Desktop
        openWindows={openWindows}
        setOpenWindows={setOpenWindows}
        userPowerActionMoreVisible={userPowerActionMoreVisible}
        setUserPowerActionMoreVisible={setUserPowerActionMoreVisible}
      />
      <DesktopTaskbar
        isAtDesktop={isAtDesktop}
        taskbarItems={taskbarItems}
        setOpenWindows={setOpenWindows}
        openWindows={openWindows}
      />
    </>
  );
}
