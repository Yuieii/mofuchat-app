import { h, type Component } from "vue";
import { Concat, Join, PascalCase, Replace, Split } from "../../../types";
import { FontAwesomeSVGNames } from "../../../types/fontawesome-icons";

type IconVariant = "solid" | "regular";

type Definition = {
    [P in IconVariant]: Component
};

function wrapComponent(definition: Partial<Definition> & Required<Pick<Definition, "solid">>): Component<{
    variant?: IconVariant
}> {
    return {
        props: {
            variant: "solid"
        },
        setup(props) {
            const variant = props.variant ?? "solid";
            return () => h(definition[variant]!);
        }
    }
}

const modules = import.meta.glob("/src/assets/fa/*.svg", {
    query: "?component",
    eager: true,
    import: "default"
});

type ComponentName = PascalCase<Join<Split<FontAwesomeSVGNames, "-"> extends [...infer A, string, string] ? A : never, "_">>;

type FontAwesome = {
    [P in ComponentName]: Component<{
        variant?: IconVariant
    }>
};

function makeExport(): FontAwesome {
    console.log(modules);
    
    const definitionsMap: Record<string, Partial<Definition>> = {};
    
    for (const key in modules) {
        const fileName = key.split("/").reverse()[0].split(".svg")[0];
        const splits = fileName.split("-");

        splits.pop();
        const variant = splits.pop()! as IconVariant;

        const componentName = splits.map(c => c[0].toUpperCase() + c.substring(1)).join("");
        const definition = definitionsMap[componentName] ??= {};
        definition[variant] = modules[key] as Component;
    }

    // @ts-ignore
    const result: FontAwesome = {};

    for (const key in definitionsMap) {
        // @ts-ignore
        result[key] = wrapComponent(definitionsMap[key]);
    }
    
    return result;
}

export const FontAwesome = makeExport();

/*
export const FontAwesome = {
    Ellipsis: wrapComponent({
        solid: Ellipsis as Component,
    }),

    Comments: wrapComponent({
        solid: Comments as Component,
    }),

    ChevronRight: wrapComponent({
        solid: ChevronRight as Component,
    }),

    GripVertical: wrapComponent({
        solid: GripVertical as Component,
    }),

    XMark: wrapComponent({
        solid: XMark as Component,
    }),

    Trash: wrapComponent({
        solid: Trash as Component,
    }),

    CircleStop: wrapComponent({
        solid: CircleStop as Component,
    }),

    AddressBook: wrapComponent({
        solid: AddressBook as Component,
    }),

};
*/