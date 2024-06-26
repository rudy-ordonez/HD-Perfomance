<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT SetTicketAction</name>
   <tag></tag>
   <elementGuidId>8d186510-8dac-4adc-b054-a84791b704e2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
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
      <name>Authorization</name>
      <type>Main</type>
      <value>${TIPWebAPI_Bearer}</value>
      <webElementGuid>6df6fae3-02f7-4bfc-8d77-deda21f015ac</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>8515a426-4b38-4468-a726-428e67fbf832</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${TIPWebAPI_Url}/api/Tag/SetTicketAction/${inventoryID}/${ticketActionID}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.TIPWebAPI_Url</defaultValue>
      <description></description>
      <id>20fbff59-b5c5-4d64-880a-31183c2a4b7d</id>
      <masked>false</masked>
      <name>TIPWebAPI_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.inventoryID</defaultValue>
      <description></description>
      <id>d75b3b4a-e6a3-4581-976a-47e6a6b6d332</id>
      <masked>false</masked>
      <name>inventoryID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ticketActionID</defaultValue>
      <description></description>
      <id>805b3a24-8c70-4a89-823a-faf12b081a24</id>
      <masked>false</masked>
      <name>ticketActionID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TIPWebAPI_Bearer</defaultValue>
      <description></description>
      <id>b18d99c0-fc78-49b1-8675-fea830c561e9</id>
      <masked>false</masked>
      <name>TIPWebAPI_Bearer</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
