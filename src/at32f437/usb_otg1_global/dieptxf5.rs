#[doc = "Register `DIEPTXF5` reader"]
pub type R = crate::R<DIEPTXF5_SPEC>;
#[doc = "Register `DIEPTXF5` writer"]
pub type W = crate::W<DIEPTXF5_SPEC>;
#[doc = "Field `INEPTXFSTADDR` reader - IN endpoint FIFO5 transmit RAM start address"]
pub type INEPTXFSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFSTADDR` writer - IN endpoint FIFO5 transmit RAM start address"]
pub type INEPTXFSTADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `INEPTXFDEP` reader - IN endpoint TxFIFO depth"]
pub type INEPTXFDEP_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFDEP` writer - IN endpoint TxFIFO depth"]
pub type INEPTXFDEP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFO5 transmit RAM start address"]
    #[inline(always)]
    pub fn ineptxfstaddr(&self) -> INEPTXFSTADDR_R {
        INEPTXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfdep(&self) -> INEPTXFDEP_R {
        INEPTXFDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF5")
            .field(
                "ineptxfstaddr",
                &format_args!("{}", self.ineptxfstaddr().bits()),
            )
            .field("ineptxfdep", &format_args!("{}", self.ineptxfdep().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIEPTXF5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint FIFO5 transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxfstaddr(&mut self) -> INEPTXFSTADDR_W<DIEPTXF5_SPEC, 0> {
        INEPTXFSTADDR_W::new(self)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxfdep(&mut self) -> INEPTXFDEP_W<DIEPTXF5_SPEC, 16> {
        INEPTXFDEP_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF5_SPEC;
impl crate::RegisterSpec for DIEPTXF5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf5::R`](R) reader structure"]
impl crate::Readable for DIEPTXF5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptxf5::W`](W) writer structure"]
impl crate::Writable for DIEPTXF5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTXF5 to value 0x0200_0400"]
impl crate::Resettable for DIEPTXF5_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0400;
}
