/**
 * https://github.com/glennflanagan/react-collapsible/issues/177
 *
 * If the inner content of the collapsible has a scrollHeight of 0
 * `onTransitionEnd` will not fire.
 *
 * To fix this we do set 'inTransition' to false if the element has 0 height value.
 */
const setInTransition = (innerRefScrollHeight) => innerRefScrollHeight !== 0;

export default setInTransition;
