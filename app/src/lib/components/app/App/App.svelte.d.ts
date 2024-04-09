import { SvelteComponent } from "svelte";
/** @slot header - Insert fixed header content, such as Skeleton's App Bar component.
     * @slot sidebarLeft - Hidden when empty. Allows you to set fixed left sidebar content.
     * @slot sidebarRight - Hidden when empty. Allows you to set fixed right sidebar content.
     * @slot pageHeader - Insert content that resides above your page content. Great for global alerts.
     * @slot pageFooter - Insert content that resides below your page content. Recommended for most layouts.
     * @slot footer - Insert fixed footer content. Not recommended for most layouts.
     */
import type { SvelteEvent } from '../../index.js';
declare const __propDef: {
    props: {
        [x: string]: any;
        /** Set `scrollbar-gutter` style.*/
        scrollbarGutter?: string | undefined;
        /** Apply arbitrary classes to the entire `#page` region.*/
        regionPage?: string | undefined;
        /** Apply arbitrary classes to the `sidebarLeft` slot container element*/
        slotSidebarLeft?: string | undefined;
        /** Apply arbitrary classes to the `sidebarRight` slot container element*/
        slotSidebarRight?: string | undefined;
        /** Apply arbitrary classes to the `pageHeader` slot container element*/
        slotPageHeader?: string | undefined;
        /** Apply arbitrary classes to the `pageContent` slot container element*/
        slotPageContent?: string | undefined;
        /** Apply arbitrary classes to the `pageFooter` slot container element*/
        slotPageFooter?: string | undefined;
    };
    events: {
        scroll: SvelteEvent<UIEvent, HTMLDivElement>;
    };
    slots: {
        sidebarLeft: {};
        pageHeader: {};
        default: {};
        pageFooter: {};
        sidebarRight: {};
    };
};
export type AppProps = typeof __propDef.props;
export type AppEvents = typeof __propDef.events;
export type AppSlots = typeof __propDef.slots;
export default class App extends SvelteComponent<AppProps, AppEvents, AppSlots> {
}
export {};
