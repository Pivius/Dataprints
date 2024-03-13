import React, { useState } from 'react'
import "./blueprint.scss"
import Navbar from '../components/navbar.jsx'

const MouseHandler = () => {
    const [isDown, setIsDown] = useState(false);
    const [position, setPosition] = useState({ x: 0, y: 0 });
    
    const handleRightClick = (e) => {
        e.preventDefault();
        OpenContextMenu(e.clientX, e.clientY);
    }

    const handleMouseDown = (e) => {
        if (e.button === 2) {
            handleRightClick(e);
        } else {
            CloseContextMenu();
            setIsDown(true);
            setPosition({ x: e.clientX, y: e.clientY });
        }
    }

    const handleMouseUp = () => {
        setIsDown(false);
    }

    const getDelta = (newX, newY) => {
        return {
            x: newX - position.x,
            y: newY - position.y,
        }
    }

    const handleMouseMove = (e) => {
        if (isDown) {
            const delta = getDelta(e.clientX, e.clientY);
            setPosition({ x: e.clientX, y: e.clientY });
        }
    }
}

export default function Blueprints() {
    return (
        <>
            <Navbar />
            <editor></editor>
            <sidebar>
                <explorer></explorer>
                <object></object>
            </sidebar>
        </>
    )
}