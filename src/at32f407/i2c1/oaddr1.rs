#[doc = "Register `OADDR1` reader"]
pub type R = crate::R<OADDR1_SPEC>;
#[doc = "Register `OADDR1` writer"]
pub type W = crate::W<OADDR1_SPEC>;
#[doc = "Field `ADDR1` reader - Own address 1"]
pub type ADDR1_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR1` writer - Own address 1"]
pub type ADDR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `ADDR1MODE` reader - Address mode"]
pub type ADDR1MODE_R = crate::BitReader;
#[doc = "Field `ADDR1MODE` writer - Address mode"]
pub type ADDR1MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:9 - Own address 1"]
    #[inline(always)]
    pub fn addr1(&self) -> ADDR1_R {
        ADDR1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Address mode"]
    #[inline(always)]
    pub fn addr1mode(&self) -> ADDR1MODE_R {
        ADDR1MODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OADDR1")
            .field("addr1mode", &format_args!("{}", self.addr1mode().bit()))
            .field("addr1", &format_args!("{}", self.addr1().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<OADDR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - Own address 1"]
    #[inline(always)]
    #[must_use]
    pub fn addr1(&mut self) -> ADDR1_W<OADDR1_SPEC, 0> {
        ADDR1_W::new(self)
    }
    #[doc = "Bit 15 - Address mode"]
    #[inline(always)]
    #[must_use]
    pub fn addr1mode(&mut self) -> ADDR1MODE_W<OADDR1_SPEC, 15> {
        ADDR1MODE_W::new(self)
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
#[doc = "Own address register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oaddr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oaddr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OADDR1_SPEC;
impl crate::RegisterSpec for OADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oaddr1::R`](R) reader structure"]
impl crate::Readable for OADDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oaddr1::W`](W) writer structure"]
impl crate::Writable for OADDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OADDR1 to value 0"]
impl crate::Resettable for OADDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
