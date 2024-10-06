import React, { useState } from 'react'
import Submit from './Submit';

import Container from 'react-bootstrap/Container';
import OutputContainer from './OutputContainer';


function FormContainer() {
    const [results, setResults] = useState(() => {
        console.log("initializing results")
    })
    const changeResult = (results) => {
        setResults(results)
        console.log(results)
    }

    return (
        <div>
            <Container className="p-3">
                <Container className="p-5 mb-4 bg-light rounded-3">
                    <h1 className="header">WikiRacerSolver</h1>
                    <Submit changeResult={changeResult} />
                </Container>
            </Container>
            {typeof results != "undefined"? <OutputContainer results={results}/> : ''}
        </div>
    )
}

export default FormContainer