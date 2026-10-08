/// <reference types="vite-plugin-pwa/client" />
import { render } from 'preact';
import { App } from './App';
import './style.css';
import '@vscode/codicons/dist/codicon.css';
import { registerSW } from 'virtual:pwa-register';

registerSW({ immediate: true });

render(<App />, document.getElementById('app') as HTMLElement);