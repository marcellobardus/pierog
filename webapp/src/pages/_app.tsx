import '../styles/global.css';
import '../styles/SearchBar.module.css';
import '../styles/SearchResults.module.css';

import type { AppProps /*, AppContext */ } from 'next/app'

function Home({ Component, pageProps }: AppProps) {
  return <>
      <div className="home">
      <h1>Clarity - Fetch the source code of any Cairo program</h1>
      <Component {...pageProps} />
    </div>
  </>
}

export default Home;