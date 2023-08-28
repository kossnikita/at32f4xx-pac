#[doc = "Register `EPT7` reader"]
pub type R = crate::R<EPT7_SPEC>;
#[doc = "Register `EPT7` writer"]
pub type W = crate::W<EPT7_SPEC>;
#[doc = "Field `EPTADDR` reader - Endpoint address"]
pub type EPTADDR_R = crate::FieldReader;
#[doc = "Field `EPTADDR` writer - Endpoint address"]
pub type EPTADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TXSTS` reader - Tx status"]
pub type TXSTS_R = crate::FieldReader;
#[doc = "Field `TXSTS` writer - Tx status"]
pub type TXSTS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TXDTS` reader - Tx data toggle synchronization"]
pub type TXDTS_R = crate::BitReader;
#[doc = "Field `TXDTS` writer - Tx data toggle synchronization"]
pub type TXDTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXTC` reader - Tx transaction completed"]
pub type TXTC_R = crate::BitReader;
#[doc = "Field `TXTC` writer - Tx transaction completed"]
pub type TXTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXF` reader - Endpoint extend function"]
pub type EXF_R = crate::BitReader;
#[doc = "Field `EXF` writer - Endpoint extend function"]
pub type EXF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRANS_TYPE` reader - Transfer type"]
pub type TRANS_TYPE_R = crate::FieldReader;
#[doc = "Field `TRANS_TYPE` writer - Transfer type"]
pub type TRANS_TYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SETUPTC` reader - Setup transaction completed"]
pub type SETUPTC_R = crate::BitReader;
#[doc = "Field `SETUPTC` writer - Setup transaction completed"]
pub type SETUPTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXSTS` reader - Rx Status"]
pub type RXSTS_R = crate::FieldReader;
#[doc = "Field `RXSTS` writer - Rx Status"]
pub type RXSTS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `RXDTS` reader - Rx data toggle synchronization"]
pub type RXDTS_R = crate::BitReader;
#[doc = "Field `RXDTS` writer - Rx data toggle synchronization"]
pub type RXDTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXTC` reader - Rx transaction completed"]
pub type RXTC_R = crate::BitReader;
#[doc = "Field `RXTC` writer - Rx transaction completed"]
pub type RXTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    pub fn eptaddr(&self) -> EPTADDR_R {
        EPTADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Tx status"]
    #[inline(always)]
    pub fn txsts(&self) -> TXSTS_R {
        TXSTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Tx data toggle synchronization"]
    #[inline(always)]
    pub fn txdts(&self) -> TXDTS_R {
        TXDTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx transaction completed"]
    #[inline(always)]
    pub fn txtc(&self) -> TXTC_R {
        TXTC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint extend function"]
    #[inline(always)]
    pub fn exf(&self) -> EXF_R {
        EXF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Transfer type"]
    #[inline(always)]
    pub fn trans_type(&self) -> TRANS_TYPE_R {
        TRANS_TYPE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setuptc(&self) -> SETUPTC_R {
        SETUPTC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Rx Status"]
    #[inline(always)]
    pub fn rxsts(&self) -> RXSTS_R {
        RXSTS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Rx data toggle synchronization"]
    #[inline(always)]
    pub fn rxdts(&self) -> RXDTS_R {
        RXDTS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx transaction completed"]
    #[inline(always)]
    pub fn rxtc(&self) -> RXTC_R {
        RXTC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    #[must_use]
    pub fn eptaddr(&mut self) -> EPTADDR_W<EPT7_SPEC, 0> {
        EPTADDR_W::new(self)
    }
    #[doc = "Bits 4:5 - Tx status"]
    #[inline(always)]
    #[must_use]
    pub fn txsts(&mut self) -> TXSTS_W<EPT7_SPEC, 4> {
        TXSTS_W::new(self)
    }
    #[doc = "Bit 6 - Tx data toggle synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn txdts(&mut self) -> TXDTS_W<EPT7_SPEC, 6> {
        TXDTS_W::new(self)
    }
    #[doc = "Bit 7 - Tx transaction completed"]
    #[inline(always)]
    #[must_use]
    pub fn txtc(&mut self) -> TXTC_W<EPT7_SPEC, 7> {
        TXTC_W::new(self)
    }
    #[doc = "Bit 8 - Endpoint extend function"]
    #[inline(always)]
    #[must_use]
    pub fn exf(&mut self) -> EXF_W<EPT7_SPEC, 8> {
        EXF_W::new(self)
    }
    #[doc = "Bits 9:10 - Transfer type"]
    #[inline(always)]
    #[must_use]
    pub fn trans_type(&mut self) -> TRANS_TYPE_W<EPT7_SPEC, 9> {
        TRANS_TYPE_W::new(self)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    #[must_use]
    pub fn setuptc(&mut self) -> SETUPTC_W<EPT7_SPEC, 11> {
        SETUPTC_W::new(self)
    }
    #[doc = "Bits 12:13 - Rx Status"]
    #[inline(always)]
    #[must_use]
    pub fn rxsts(&mut self) -> RXSTS_W<EPT7_SPEC, 12> {
        RXSTS_W::new(self)
    }
    #[doc = "Bit 14 - Rx data toggle synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn rxdts(&mut self) -> RXDTS_W<EPT7_SPEC, 14> {
        RXDTS_W::new(self)
    }
    #[doc = "Bit 15 - Rx transaction completed"]
    #[inline(always)]
    #[must_use]
    pub fn rxtc(&mut self) -> RXTC_W<EPT7_SPEC, 15> {
        RXTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "endpoint 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPT7_SPEC;
impl crate::RegisterSpec for EPT7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ept7::R`](R) reader structure"]
impl crate::Readable for EPT7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ept7::W`](W) writer structure"]
impl crate::Writable for EPT7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EPT7 to value 0"]
impl crate::Resettable for EPT7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
