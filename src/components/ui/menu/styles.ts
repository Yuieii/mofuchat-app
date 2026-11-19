export const menuItemCommonStyles = [
    "px-3",
    "py-1.5",
    "text-sm",
    "hover:bg-accent-500",
    "hover:text-white", "hover:fill-white",
    "rounded-lg",
    "min-w-35",
];

export const popoverCommonStyles = [
    "mf-menu-surface",
    "open:flex", "rounded-xl", 
    "border-1", "border-accent-300",
    "shadow-[0_0_0.5em]", "shadow-accent-500",
    "overflow-y-auto", "max-h-[85vh]", 
];

export const menuContentCommonStyles = [
    ...popoverCommonStyles,
    "flex-col", "gap-1", "p-1",
];