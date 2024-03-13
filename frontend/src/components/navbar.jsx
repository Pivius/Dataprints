import React, { useEffect, useState } from 'react';
import {KeyMapper} from '../classes/keymapper.tsx'
import Dropdown from '../components/dropdown.jsx'
import './styles/navbar.scss'

const navOptions = [
    {label: "File", options: [
        {label: "New", isDropdown: false, onClick: () => console.log("New"), hotKey: new KeyMapper({key: "n", ctrlKey: true})},
        {label: "Open", isDropdown: false, onClick: () => console.log("Open"), hotKey: new KeyMapper({key: "o", ctrlKey: true})},
        {label: "Save", isDropdown: false, onClick: () => console.log("Save"), hotKey: new KeyMapper({key: "s", ctrlKey: true}), separator: true},
        {label: "Import", isDropdown: false, onClick: () => console.log("Import"), hotKey: new KeyMapper({key: "i", ctrlKey: true})},
        {label: "Export", isDropdown: false, onClick: () => console.log("Export"), hotKey: new KeyMapper({key: "e", ctrlKey: true})},
    ]},
    {label: "Edit", options: [
        {label: "Undo", isDropdown: false, onClick: () => console.log("Undo"), hotKey: new KeyMapper({key: "z", ctrlKey: true})},
        {label: "Redo", isDropdown: false, onClick: () => console.log("Redo"), hotKey: new KeyMapper({key: "y", ctrlKey: true}), separator: true},
        {label: "Cut", isDropdown: false, onClick: () => console.log("Cut"), hotKey: new KeyMapper({key: "x", ctrlKey: true})},
        {label: "Copy", isDropdown: false, onClick: () => console.log("Copy"), hotKey: new KeyMapper({key: "c", ctrlKey: true})},
        {label: "Paste", isDropdown: false, onClick: () => console.log("Paste"), hotKey: new KeyMapper({key: "v", ctrlKey: true})},
    ]},
    {label: "Help", options: [
        {label: "About", isDropdown: false, onClick: () => console.log("About")},
    ]}
]

export default function Navbar() {
    return (
        <div className="nav">
            {navOptions.map((option) => (
                <Dropdown 
                    key={option.label} 
                    label={option.label} 
                    options={option.options} 
                />
            ))}
        </div>
    )
}