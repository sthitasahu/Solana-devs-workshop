import { useMemo } from "react";
import {
  ConnectionProvider,
  WalletProvider,
} from "@solana/wallet-adapter-react";
import { WalletAdapterNetwork } from "@solana/wallet-adapter-base";
import {
  WalletModalProvider,
  WalletMultiButton,
} from "@solana/wallet-adapter-react-ui";
import { clusterApiUrl } from "@solana/web3.js";
import "./App.css";

function App() {
  
  const network=WalletAdapterNetwork.Devnet
  console.log(network)
  const endpoint = useMemo(() => clusterApiUrl(network), [network]);

  const wallets = useMemo(
    () => [
      // if desired, manually define specific/custom wallets here (normally not required)
      // otherwise, the wallet-adapter will auto detect the wallets a user's browser has available
    ],
    [network],
  );

  return (
    <>
       <ConnectionProvider endpoint={endpoint}>
          <WalletProvider wallets={wallets} autoConnect>
             <WalletModalProvider>
              <WalletMultiButton/>
                <h1>Welcome to the counter dapp built on solana</h1>
             </WalletModalProvider>
          </WalletProvider>
       </ConnectionProvider>
        
    </>
  )
}

export default App
