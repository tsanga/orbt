import * as styles from "./index.css";
import Logo from "@assets/svg/logo-long.svg";
import PlanetsBackground from "@assets/svg/planets-bg.svg";
import * as theme from "@theme/theme.css";

type Props = {
  children: React.ReactNode;
};

export default function IsolatedForm({ children }: Props) {
  return (
    <div className={styles.container}>
      <div className={styles.overlay.outer}>
        <PlanetsBackground
          className={styles.overlay.inner}
          height="100%"
          width="100%"
        />
      </div>
      <header className={styles.header}>
        <Logo />
      </header>
      <main className={styles.main}>{children}</main>
      <footer className={styles.footer}>
        <h1>FOoter</h1>
      </footer>
    </div>
  );
}
