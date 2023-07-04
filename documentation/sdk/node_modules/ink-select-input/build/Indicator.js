"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const React = require("react");
const ink_1 = require("ink");
const figures = require("figures");
const Indicator = ({ isSelected = false }) => (React.createElement(ink_1.Box, { marginRight: 1 }, isSelected ? React.createElement(ink_1.Text, { color: "blue" }, figures.pointer) : React.createElement(ink_1.Text, null, " ")));
exports.default = Indicator;
