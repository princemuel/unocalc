import { registerSW } from "virtual:pwa-register";

let swActivated = false;

const period = 1.5 * 24 * 60 * 60 * 1000; // check for updates every day

let refreshSW: ( ( reloadPage?: boolean ) => Promise<void> ) | undefined;

const _ = () => refreshSW?.( true );

registerSW( {
  immediate: true,
  onOfflineReady() {
    console.info( "[Vite Plugin PWA] PWA application ready to work offline" );
  },
  onNeedRefresh() {
    console.info( "[Vite Plugin PWA] PWA application needs an update" );
  },
  onRegisteredSW( swUrl, r ) {
    if ( period <= 0 ) return;
    if ( r?.active?.state === "activated" ) {
      swActivated = true;
      sync( period, swUrl, r );
      console.info(
        `[Vite Plugin PWA] SW registered: ${ swUrl } after activation`,
      );
    } else if ( r?.installing ) {
      r.installing.addEventListener( "statechange", ( e ) => {
        const sw = e.target as ServiceWorker;
        swActivated = sw.state === "activated";
        if ( swActivated ) sync( period, swUrl, r );
        console.info(
          `[Vite Plugin PWA] SW registered: ${ swUrl } after installing`,
        );
      } );
    }
  },
} );

/**
 * This `sync` function will check for updates
 * according to the period interval specified
 */
function sync( period: number, url: string, r: ServiceWorkerRegistration ) {
  if ( period <= 0 ) return;

  setInterval( async () => {
    if ( "connection" in navigator && !navigator.onLine ) return;

    const response = await fetch( url, {
      cache: "no-store",
      headers: { cache: "no-store", "cache-control": "no-cache" },
    } );
    if ( response?.status === 200 ) await r.update();
  }, period );
}
