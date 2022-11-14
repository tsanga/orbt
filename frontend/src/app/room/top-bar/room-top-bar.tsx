import * as styles from "./room-tar-bar.css";
import Logo from "@assets/svg/logo-long.svg";

export default function TopBar() {
  return (
    <nav className={styles.nav}>
      <Logo />
    </nav>
  );
}
