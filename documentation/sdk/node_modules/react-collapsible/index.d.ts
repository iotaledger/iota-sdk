import * as React from "react";

declare class Collapsible extends React.Component<CollapsibleProps> {}

export interface CollapsibleProps extends React.HTMLProps<Collapsible> {
  transitionTime?: number;
  transitionCloseTime?: number | null;
  triggerTagName?: string;
  easing?: string;
  open?: boolean;
  containerElementProps?: object;
  classParentString?: string;
  openedClassName?: string;
  triggerStyle?: null | React.CSSProperties;
  triggerClassName?: string;
  triggerOpenedClassName?: string;
  triggerElementProps?: object;
  contentElementId?: string;
  contentOuterClassName?: string;
  contentInnerClassName?: string;
  accordionPosition?: string | number;
  handleTriggerClick?: (accordionPosition?: string | number) => void;
  onOpen?: () => void;
  onClose?: () => void;
  onOpening?: () => void;
  onClosing?: () => void;
  onTriggerOpening?: () => void;
  onTriggerClosing?: () => void;
  trigger: string | React.ReactElement<any>;
  triggerWhenOpen?: string | React.ReactElement<any>;
  triggerDisabled?: boolean;
  lazyRender?: boolean;
  overflowWhenOpen?:
    | "hidden"
    | "visible"
    | "auto"
    | "scroll"
    | "inherit"
    | "initial"
    | "unset";
  contentHiddenWhenClosed?: boolean;
  triggerSibling?: string | React.ReactElement<any>;
  className?: string;
  tabIndex?: number;
  contentContainerTagName?: string;
}

export default Collapsible;
