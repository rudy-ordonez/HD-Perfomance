<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET GetAvailableProblemTypes</name>
   <tag></tag>
   <elementGuidId>f0499af8-27a5-454f-a439-3ad7837ab8f8</elementGuidId>
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
      <value>${HHD_API_Bearer}</value>
      <webElementGuid>4de68931-2ebd-42a4-a6b1-224ab975314b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>9dfd2827-560d-4796-80ac-58cbbe2acc20</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${HHD_API_Url}/api/TicketTypeTemplate/GetAvailableProblemTypes</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Url</defaultValue>
      <description></description>
      <id>81e785a2-0fd2-4b11-9c9d-70e4959d4c1d</id>
      <masked>false</masked>
      <name>HHD_API_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Bearer</defaultValue>
      <description></description>
      <id>fe720477-53de-4a91-9825-81811d0466ac</id>
      <masked>false</masked>
      <name>HHD_API_Bearer</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
