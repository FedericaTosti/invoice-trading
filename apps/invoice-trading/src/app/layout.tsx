import './global.css';

import { Inter } from 'next/font/google';

export const metadata = {
  title: 'Invoice Trading on Blockchain',
  description: 'Made with love by the Invoice Trading team :)',
};

const inter = Inter({ subsets: ['latin'] });

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>{children}</body>
    </html>
  );
}
