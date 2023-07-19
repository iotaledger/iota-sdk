# Inspired by
# https://github.com/tqdm/tqdm.github.io/blob/main/pydoc_markdown_tqdm.py
# as reference

from pydoc_markdown.contrib.processors.pydocmd import PydocmdProcessor
import re
from functools import partial

sub = partial(re.sub, flags=re.M)


class IotaProcessor(PydocmdProcessor):
    def _process(self, node):
        if not getattr(node, "docstring", None):
            return

        c = node.docstring.content
        # join long lines ending in escape (\)
        c = sub(r"\\\n\s*", "", c)
        # convert parameter lists to markdown list
        c = sub(r"^(\w+)\s{1,}(:.*?)$", r"* __\1__*\2*  ", c)
        # Convert "Parameters" and "Returns" to <h4>
        c = sub(r"^(.+?)\n[-]{4,}$", r"#### \1\n", c)
        node.docstring.content = c

        return super()._process(node)
