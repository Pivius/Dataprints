import React, { useState } from 'react'
import "./blueprint.scss"
import Navbar from '../components/navbar.jsx'

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