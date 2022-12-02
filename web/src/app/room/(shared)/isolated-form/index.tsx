"use client";

import * as styles from "./index.css";
import Logo from "@assets/svg/logo-long.svg";
import PlanetsBackground from "@assets/svg/planets-bg.svg";
import TsangaLogo from "@assets/svg/tsanga.svg";
import * as theme from "@theme/theme.css";
import Link from "next/link";
import TwitterIcon from "@assets/svg/icon/twitter.svg";
import GitHubIcon from "@assets/svg/icon/github.svg";
import InstagramIcon from "@assets/svg/icon/instagram.svg";

type Props = {
  children: React.ReactNode;
};

function SocialButton({ icon, href }: { icon: React.ReactNode; href: string }) {
  return (
    <Link className={styles.socialItem} href={href} target="_blank">
      {icon}
    </Link>
  );
}

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
        <Link
          href={
            new URL(
              process.env.NEXT_PUBLIC_ORG_URL || "https://github.com/tsanga"
            )
          }
          target="_blank"
          className={styles.orgLogo}
        >
          <TsangaLogo />
        </Link>
        <ul className={styles.socialList}>
          {[
            {
              icon: <GitHubIcon />,
              href:
                process.env.NEXT_PUBLIC_GITHUB_URL ||
                "https://github.com/tsanga/orbt",
            },
            {
              icon: <TwitterIcon />,
              href:
                process.env.NEXT_PUBLIC_TWITTER_URL ||
                "https://twitter.com/bizurre",
            },
            {
              icon: <InstagramIcon />,
              href:
                process.env.NEXT_PUBLIC_INSTAGRAM_URL ||
                "https://instagram.com/jonahseguin",
            },
          ].map(({ icon, href }) => (
            <SocialButton icon={icon} href={href} key={href} />
          ))}
        </ul>
      </footer>
    </div>
  );
}
