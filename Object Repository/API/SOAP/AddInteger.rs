<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddInteger</name>
   <tag></tag>
   <elementGuidId>2e9a0b5d-7bdd-4307-86f1-54733ee78155</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>SOAPAction</name>
      <type>Main</type>
      <value>http://tempuri.org/SOAP.Demo.AddInteger</value>
      <webElementGuid>ed84b092-5f39-4f57-a040-dbd69bfdc41a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
      <webElementGuid>768a6c83-a7a7-419a-9b3f-a86ae8f6e388</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <path></path>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tem=&quot;http://tempuri.org&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;tem:AddInteger>
         &lt;tem:Arg1>10&lt;/tem:Arg1>
         &lt;tem:Arg2>10&lt;/tem:Arg2>
      &lt;/tem:AddInteger>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>https://www.crcind.com:443/csp/samples/SOAP.Demo.cls</soapServiceEndpoint>
   <soapServiceFunction>AddInteger</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <validationSteps>
      <id>c1745d88-d818-49c0-8b88-1ea398237ed3</id>
      <name>SOAP Schema</name>
      <type>XML_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>/Users/fajri/Desktop/XMLSchema.txt</data>
      <activate>true</activate>
   </validationSteps>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress>https://www.crcind.com/csp/samples/SOAP.Demo.CLS?WSDL=1</wsdlAddress>
</WebServiceRequestEntity>
