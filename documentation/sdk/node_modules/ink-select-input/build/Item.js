"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const React = require("react");
const ink_1 = require("ink");
const Item = ({ isSelected = false, label }) => (React.createElement(ink_1.Text, { color: isSelected ? 'blue' : undefined }, label));
exports.default = Item;
