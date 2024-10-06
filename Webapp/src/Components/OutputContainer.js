import React from 'react'
import Submit from './Submit';
import ListGroup from 'react-bootstrap/ListGroup';
import Container from 'react-bootstrap/Container';
import { ArrowRight } from 'react-bootstrap-icons';

function OutputContainer(props) {
    return (
        <Container className="p-3">
            <Container className="p-5 mb-4 bg-light rounded-3">
                Found path
                <ListGroup horizontal>
                    {
                        //JSON.stringify(props.results.path, null, 2)
                        props.results.path.map((page) => (
                            <div>
                                <ListGroup.Item variant="success">
                                    <ArrowRight />
                                    {page}
                                </ListGroup.Item>
                            </div>
                        ))
                    }<br />
                </ListGroup>
            </Container>
        </Container>
    )
}

export default OutputContainer