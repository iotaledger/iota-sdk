import ExecutionEnvironment from '@docusaurus/ExecutionEnvironment';

export default (function () {
  if (!ExecutionEnvironment.canUseDOM) {
    return null;
  }
  return {
    onRouteUpdate({location}) {
      _paq.push(['setCustomUrl', location.pathname]);
      _paq.push(['setDocumentTitle', document.title]);
      _paq.push(['trackPageView']);
    },
  };
})();
