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

WebUI.maximizeWindow()

WebUI.click(findTestObject('Ticket Creation Routing/b_Admin'))

WebUI.click(findTestObject('Ticket Creation Routing/p_Ticket Routing'))

WebUI.click(findTestObject('Ticket Creation Routing/button_Create Rule'))

WebUI.setText(findTestObject('Ticket Creation Routing/input__fieldName'), 'Routing Rule')

WebUI.click(findTestObject('Ticket Creation Routing/button_Add Condition'))

WebUI.selectOptionByValue(findTestObject('Ticket Creation Routing/Problem Type'), '3: 5-System', true)

WebUI.selectOptionByValue(findTestObject('Ticket Creation Routing/Is'), '1', true)

WebUI.click(findTestObject('Ticket Creation Routing/div_one_k-multiselect-wrap k-floatwrap'))

WebUI.click(findTestObject('Ticket Creation Routing/li_High'))

WebUI.click(findTestObject('Ticket Creation Routing/button_Add Condition'))

WebUI.selectOptionByValue(findTestObject('Ticket Creation Routing/Priority'), '4: 22-System', true)

WebUI.selectOptionByValue(findTestObject('Ticket Creation Routing/select_Is Is Not Contains Does Not Contain Starts With Ends With Is Blank'),
	'1', true)

WebUI.click(findTestObject('Ticket Creation Routing/div_High_k-multiselect-wrap k-floatwrap'))

WebUI.click(findTestObject('Ticket Creation Routing/span__SD Hardware Prob Typ 4.3'))

WebUI.selectOptionByValue(findTestObject('Ticket Creation Routing/select_Unassigned------------------  Addy M_541135'),
	'11113', true)

WebUI.click(findTestObject('Ticket Creation Routing/input_Assign to Service Group_k-12c421cf-23_bb1726'))

WebUI.click(findTestObject('Ticket Creation Routing/li_Software Tickets'))

WebUI.click(findTestObject('Ticket Creation Routing/span_Set Status To_gh-slider-disabled round'))

WebUI.selectOptionByValue(findTestObject('Ticket Creation Routing/select_Open  Removed  Request Approval'), '10', true)

WebUI.click(findTestObject('Ticket Creation Routing/button_Create'))