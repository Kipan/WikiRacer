import React, { useState } from 'react'
import Col from 'react-bootstrap/Col';
import Form from 'react-bootstrap/Form';
import Row from 'react-bootstrap/Row';
import axios from "axios";
import Button from 'react-bootstrap/Button';
import Spinner from 'react-bootstrap/Spinner';
import OutputContainer from './OutputContainer';

function Submit({ changeResult }) {

    const [start, setStart] = useState(() => {
        console.log("initializing start")
    })

    const [target, setTarget] = useState(() => {
        console.log("initializing target")
    })

    const [loading, setLoading] = useState(false)

    const onChangeResult = (results) => {
        changeResult(results)
    }

    const handleSubmit = async (e) => {
        e.preventDefault()
        const uri = 'http://localhost:8080/submit'
        try {
            setLoading(true)
            const response = await axios.post(uri, null, {
                params: {
                    start,
                    target
                }
            })
            setLoading(false)
            console.log(response.data);
            onChangeResult(response.data)
        } catch (error) {
            console.log(error);
        }
    }


    return (
        <div>
            <Form>

                <Form.Group as={Row} className="mb-3" controlId="formFrom">
                    <Form.Label column sm="2">
                        From
                    </Form.Label>
                    <Col sm="10">
                        <Form.Control type="text" placeholder='Cheesecake' autocomplete="off" onChange={e => setStart(e.target.value)} />
                    </Col>


                </Form.Group>

                <Form.Group as={Row} className="mb-3" controlId="formTo">
                    <Form.Label column sm="2">
                        To
                    </Form.Label>
                    <Col sm="10">
                        <Form.Control type="text" placeholder='Queens' autocomplete="off" onChange={e => setTarget(e.target.value)} />
                    </Col>
                </Form.Group>
                <Row className="justify-content-md-center">
                    <Col xs lg="2">
                        <Button type="button" disabled={loading ? true : false} onClick={handleSubmit}>
                            {loading ? <Spinner
                                as="span"
                                animation="grow"
                                size="sm"
                                role="status"
                                aria-hidden="true"
                            /> :
                                'Submit'}
                        </Button>
                    </Col>
                </Row>
            </Form>
        </div>
    )
}

export default Submit