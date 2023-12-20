import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.click(findTestObject('Object Repository/Create Ticket/Create SQL Ticket/h1_Create Ticket'))

WebUI.click(findTestObject('Create Ticket/Create SQL Ticket/p_Counterparts'))

WebUI.setText(findTestObject('Object Repository/Create Ticket/Create SQL Ticket/input_Summary_summary'), 'Katalon Automated Ticket')

WebUI.setText(findTestObject('Object Repository/Create Ticket/Create SQL Ticket/textarea_Description_description'), 'The Theory of Relativity, proposed by the Jewish physicist Albert Einstein (1879-1955) in the early part of the 20th century, is one of the most significant scientific advances of our time. Although the concept of relativity was not introduced by Einstein, his major contribution was the recognition that the speed of light in a vacuum is constant and an absolute physical boundary for motion. This does not have a major impact on a person\'s day-to-day life since we travel at speeds much slower than light speed. For objects travelling near light speed, however, the theory of relativity states that objects will move slower and shorten in length from the point of view of an observer on Earth. Einstein also derived the famous equation, E = mc2, which reveals the equivalence of mass and energy. ')

WebUI.click(findTestObject('Create Ticket/Create SQL Ticket/svg_Next'))

WebUI.click(findTestObject('Object Repository/Create Ticket/Create SQL Ticket/a_Clickheretorecordittothisticket'))

WebUI.setText(findTestObject('Object Repository/Create Ticket/Create SQL Ticket/input_TagNumber_assetTagNumber'), tag)

WebUI.click(findTestObject('Object Repository/Create Ticket/Create SQL Ticket/svg_TagNumber_svg-inline--fa fa-check-squar_932a25'))

WebUI.click(findTestObject('Object Repository/Create Ticket/Create SQL Ticket/button_Yes'))

WebUI.click(findTestObject('Object Repository/Create Ticket/Create SQL Ticket/svg_Hardware Details_svg-inline--fa fa-arro_ed1b8e'))

WebUI.click(findTestObject('Object Repository/Create Ticket/Create SQL Ticket/svg_Submit'))

WebUI.click(findTestObject('Object Repository/Create Ticket/Create SQL Ticket/span_Close Modal'))

