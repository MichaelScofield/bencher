import React from "react"
import { Section, Columns, Button } from "react-bulma-components"

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome"
import {
  faPlusCircle,
  faArrowCircleUp,
} from "@fortawesome/free-solid-svg-icons"

import Element from "../flow/elements/Element"

const Page = (props: {
  subflow: any
  handleElement: Function
  handleVariable: Function
  getSubflow: Function
}) => {
  function getVariable(id: string): any {
    return props?.subflow?.variables?.[id]
  }

  return (
    <Columns>
      {props?.subflow?.order?.map((elementId: any, index: number) => {
        let element = props.subflow.elements?.[elementId]
        return (
          <React.Fragment key={index}>
            <Columns.Column size={12} narrow={true}>
              <Element
                element={element}
                handleElement={props.handleElement}
                handleVariable={props.handleVariable}
                getVariable={getVariable}
                context={{
                  parent: props.subflow.parent,
                  current: props.subflow.id,
                }}
                getSubflow={props.getSubflow}
              />
            </Columns.Column>
            {element?.type && element.type !== "output" && (
              <Columns.Column size={12} narrow={true}>
                <Button
                  color="primary"
                  size="medium"
                  fullwidth={true}
                  inverted={true}
                  title="Add Element"
                  onClick={(event: any) => console.log("TODO add Element")}
                >
                  <FontAwesomeIcon icon={faPlusCircle} size="2x" />
                </Button>
              </Columns.Column>
            )}
          </React.Fragment>
        )
      })}
      <Columns.Column size={12}>
        <Section>
          <br />
        </Section>
      </Columns.Column>
      <Columns.Column>
        <Button
          color="primary"
          size="medium"
          fullwidth={true}
          inverted={true}
          title="Jump to top of page"
          onClick={(event: any) => {
            event.preventDefault()
            window.scrollTo(0, 0)
          }}
        >
          <FontAwesomeIcon icon={faArrowCircleUp} size="2x" />
        </Button>
      </Columns.Column>
    </Columns>
  )
}

export default Page
